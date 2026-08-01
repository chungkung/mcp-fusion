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
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| format!("创建加密器失败: {e}"))?;

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
    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &combined,
    ))
}

/// 解密敏感数据（AES-256-GCM）
/// 输入为 Base64 编码的密文
pub fn decrypt(encrypted: &str, passphrase: &str) -> Result<String, String> {
    let key = derive_key(passphrase);
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| format!("创建解密器失败: {e}"))?;

    let combined = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encrypted)
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

// ============================================================
// 日志脱敏 — 防止敏感数据（token/密码/密钥）泄露到日志
// ============================================================

/// 敏感字段关键词（不区分大小写匹配）
const SENSITIVE_KEYWORDS: &[&str] = &[
    "api_key",
    "apikey",
    "api-key",
    "token",
    "secret",
    "password",
    "passwd",
    "passphrase",
    "authorization",
    "auth",
    "credential",
    "private_key",
    "private-key",
    "privatekey",
    "access_key",
    "access-key",
    "accesskey",
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
                            let value_start =
                                after_key + (result[after_key..].find(rest_no_prefix).unwrap_or(0));
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
