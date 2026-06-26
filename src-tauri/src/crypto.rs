// ============================================================
// 加密模块 — AES-256-GCM 加密 / SHA-256 哈希 / 密钥派生
// ============================================================

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::Rng;
use sha2::{Digest, Sha256};

/// 加密密钥（32 字节 = AES-256）
const KEY_SIZE: usize = 32;

/// 从密码短语派生加密密钥（SHA-256 哈希）
fn derive_key(phrase: &str) -> [u8; KEY_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(phrase.as_bytes());
    // 加盐防止彩虹表攻击
    hasher.update(b"mcp-fusion-salt-v1");
    let result = hasher.finalize();
    let mut key = [0u8; KEY_SIZE];
    key.copy_from_slice(&result[..KEY_SIZE]);
    key
}

/// 加密敏感数据（AES-256-GCM）
/// 返回 Base64 编码的密文（nonce + ciphertext 拼接）
pub fn encrypt(plaintext: &str, passphrase: &str) -> Result<String, String> {
    let key = derive_key(passphrase);
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| format!("创建加密器失败: {e}"))?;

    let mut rng = rand::thread_rng();
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| format!("加密失败: {e}"))?;

    // nonce (12 bytes) + ciphertext → Base64
    let mut combined = Vec::with_capacity(12 + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);
    Ok(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &combined))
}

/// 解密敏感数据（AES-256-GCM）
/// 输入为 Base64 编码的密文
pub fn decrypt(encrypted: &str, passphrase: &str) -> Result<String, String> {
    let key = derive_key(passphrase);
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|e| format!("创建解密器失败: {e}"))?;

    let combined = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        encrypted,
    )
    .map_err(|e| format!("Base64 解码失败: {e}"))?;

    if combined.len() < 12 {
        return Err("密文数据损坏".to_string());
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| format!("解密失败: {e}"))?;

    String::from_utf8(plaintext).map_err(|e| format!("UTF-8 解码失败: {e}"))
}

/// SHA-256 哈希（用于审计日志链）
pub fn sha256_hex(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}

/// 使用 HMAC-SHA256 派生密钥（用于 API Key 签名）
#[cfg(test)]
pub fn hmac_sha256_hex(data: &str, key: &str) -> String {
    use sha2::Sha256;

    // 简化版 HMAC: HMAC(K, m) = H((K ⊕ opad) || H((K ⊕ ipad) || m))
    const BLOCK_SIZE: usize = 64;
    let ipad = [0x36u8; BLOCK_SIZE];
    let opad = [0x5cu8; BLOCK_SIZE];

    let mut key_bytes = [0u8; BLOCK_SIZE];
    let key_raw = key.as_bytes();
    if key_raw.len() > BLOCK_SIZE {
        let hash = Sha256::digest(key_raw);
        key_bytes[..32].copy_from_slice(&hash);
    } else {
        key_bytes[..key_raw.len()].copy_from_slice(key_raw);
    }

    let mut inner = Sha256::new();
    for i in 0..BLOCK_SIZE {
        inner.update(&[key_bytes[i] ^ ipad[i]]);
    }
    inner.update(data.as_bytes());
    let inner_hash = inner.finalize();

    let mut outer = Sha256::new();
    for i in 0..BLOCK_SIZE {
        outer.update(&[key_bytes[i] ^ opad[i]]);
    }
    outer.update(&inner_hash);
    hex::encode(outer.finalize())
}

// ============================================================
// 日志脱敏 — 防止敏感数据（token/密码/密钥）泄露到日志
// ============================================================

/// 敏感字段关键词（不区分大小写匹配）
const SENSITIVE_KEYWORDS: &[&str] = &[
    "api_key", "apikey", "api-key",
    "token", "secret", "password", "passwd", "passphrase",
    "authorization", "auth", "credential",
    "private_key", "private-key", "privatekey",
    "access_key", "access-key", "accesskey",
];

/// 脱敏日志内容：将敏感字段的值替换为 `[REDACTED]`。
/// 支持 JSON 格式（`"key": "value"` → `"key": "[REDACTED]"`）和
/// URL 查询格式（`key=value` → `key=[REDACTED]`）。
pub fn sanitize_log(input: &str) -> String {
    let mut result = input.to_string();
    let double_quote: char = '"';

    for keyword in SENSITIVE_KEYWORDS {
        let lower = result.to_lowercase();
        let kw_lower = keyword.to_lowercase();

        let mut search_start = 0;
        while let Some(pos) = lower[search_start..].find(&kw_lower) {
            let abs_pos = search_start + pos;
            let after_key = abs_pos + kw_lower.len();

            // JSON 格式: "key": "value" 或 "key":"value"
            // 关键词是 JSON key 的值部分，所以 after_key 指向 key 的闭合引号
            if let Some(rest) = result[after_key..]
                .trim_start()
                .strip_prefix(double_quote)
                .and_then(|s| s.trim_start().strip_prefix(':'))
            {
                let rest_trimmed = rest.trim_start();
                if rest_trimmed.starts_with(double_quote) {
                    // 计算 value 起始位置（在原始 result 中的字节偏移）
                    let value_start = result.len() - rest_trimmed.len();
                    if let Some(rel_end) = rest_trimmed[1..].find(double_quote) {
                        let abs_end = value_start + 1 + rel_end + 1;
                        let redacted = format!("{double_quote}[REDACTED]{double_quote}");
                        result.replace_range(value_start..abs_end, &redacted);
                        search_start = value_start + redacted.len();
                        continue;
                    }
                }
            }

            // URL 查询格式: key=value& 或 key=value
            if let Some(rest) = result[after_key..].strip_prefix('=') {
                let rest_trimmed = rest.trim_start();
                let value_start = after_key + 1 + (rest.len() - rest_trimmed.len());
                let end = rest_trimmed
                    .find(|c: char| c == '&' || c == ' ' || c == '\n' || c == '\r')
                    .map(|p| value_start + 1 + (rest.len() - rest_trimmed.len()) + p)
                    .unwrap_or(result.len());
                result.replace_range(value_start..end, "[REDACTED]");
                search_start = value_start + "[REDACTED]".len();
                continue;
            }

            // Bearer token / Authorization header
            if kw_lower == "authorization" || kw_lower == "auth" {
                if let Some(rest) = result[after_key..].trim_start().strip_prefix(':') {
                    let rest = rest.trim_start();
                    let prefixes = ["Bearer ", "bearer ", "Basic ", "basic "];
                    let mut found = false;
                    for prefix in &prefixes {
                        if let Some(rest_no_prefix) = rest.strip_prefix(prefix) {
                            let value_start = after_key
                                + (result[after_key..].find(rest_no_prefix).unwrap_or(0));
                            let end = rest_no_prefix
                                .find(|c: char| {
                                    c == double_quote || c == '\'' || c == ',' || c == '\n'
                                })
                                .map(|p| value_start + p)
                                .unwrap_or(result.len());
                            result.replace_range(value_start..end, "[REDACTED]");
                            search_start = value_start + "[REDACTED]".len();
                            found = true;
                            break;
                        }
                    }
                    if found {
                        continue;
                    }
                }
            }

            search_start = after_key;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let plaintext = "sk-abc123def456";
        let passphrase = "test-passphrase";
        let encrypted = encrypt(plaintext, passphrase).unwrap();
        let decrypted = decrypt(&encrypted, passphrase).unwrap();
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_encrypt_different_passphrases() {
        let plaintext = "secret-data";
        let encrypted = encrypt(plaintext, "pass-a").unwrap();
        let result = decrypt(&encrypted, "pass-b");
        assert!(result.is_err());
    }

    #[test]
    fn test_sha256_hex_consistency() {
        let hash1 = sha256_hex("hello");
        let hash2 = sha256_hex("hello");
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, sha256_hex("world"));
    }

    #[test]
    fn test_hmac_consistency() {
        let sig1 = hmac_sha256_hex("message", "key");
        let sig2 = hmac_sha256_hex("message", "key");
        assert_eq!(sig1, sig2);
        assert_ne!(sig1, hmac_sha256_hex("message", "different-key"));
    }

    // ============================================================
    // sanitize_log 测试
    // ============================================================

    #[test]
    fn test_sanitize_json_api_key() {
        let input = r#"{"api_key": "sk-abc123def456", "name": "test"}"#;
        let result = sanitize_log(input);
        assert!(!result.contains("sk-abc123def456"));
        assert!(result.contains("[REDACTED]"));
        assert!(result.contains(r#""name": "test""#));
    }

    #[test]
    fn test_sanitize_json_token() {
        let input = r#"{"token": "my-secret-token-12345"}"#;
        let result = sanitize_log(input);
        assert!(!result.contains("my-secret-token"));
        assert!(result.contains("[REDACTED]"));
    }

    #[test]
    fn test_sanitize_json_password() {
        let input = r#"{"password": "P@ssw0rd!"}"#;
        let result = sanitize_log(input);
        assert!(!result.contains("P@ssw0rd!"));
        assert!(result.contains("[REDACTED]"));
    }

    #[test]
    fn test_sanitize_url_format() {
        let input = "api_key=sk-abc123&name=test";
        let result = sanitize_log(input);
        assert!(!result.contains("sk-abc123"));
        assert!(result.contains("[REDACTED]"));
        assert!(result.contains("name=test"));
    }

    #[test]
    fn test_sanitize_authorization_bearer() {
        let input = r#"Authorization: Bearer eyJhbGciOiJIUzI1NiJ9"#;
        let result = sanitize_log(input);
        assert!(!result.contains("eyJhbGci"));
        assert!(result.contains("[REDACTED]"));
    }

    #[test]
    fn test_sanitize_no_sensitive_data() {
        let input = "Processing workflow 'test' with 5 nodes";
        let result = sanitize_log(input);
        assert_eq!(result, input);
    }

    #[test]
    fn test_sanitize_empty_string() {
        let result = sanitize_log("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_sanitize_secret() {
        let input = r#"{"client_secret": "abcdef1234567890"}"#;
        let result = sanitize_log(input);
        assert!(!result.contains("abcdef1234567890"));
        assert!(result.contains("[REDACTED]"));
    }
}