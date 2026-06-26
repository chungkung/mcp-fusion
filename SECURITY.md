# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

**Do NOT open a public issue for security vulnerabilities.**

Instead, please report security issues to:

- **Email**: [security@mcp-fusion.app](mailto:security@mcp-fusion.app)
- **PGP Key**: [Download](https://keys.openpgp.org/search?q=security%40mcp-fusion.app)

We will acknowledge receipt within 48 hours and provide a timeline for resolution within 5 business days.

## Security Features

MCP Fusion implements the following security measures:

### Encryption
- **AES-256-GCM** for sensitive data at rest (server environment variables)
- **HMAC-SHA256** for API key verification
- **SHA-256** for audit log chain integrity

### Authentication & Authorization
- **RBAC** with three roles: Admin, Developer, Viewer
- **API Key** authentication with rate limiting
- **Encrypted key storage** in SQLite

### Runtime Protection
- **Rate limiter** prevents brute-force and DoS attacks
- **Circuit breaker** isolates failing services
- **Input sanitization** for log messages and user data
- **Idempotency keys** prevent duplicate execution

## Best Practices

When deploying MCP Fusion:

1. **Set encryption key**: `export MCP_FUSION_ENCRYPTION_KEY="your-random-key"`
2. **Set API key**: Generate via Settings > Permissions
3. **Change default role**: Set to `viewer` for production
4. **Enable audit logging**: Configure log retention in Settings
5. **Keep updated**: Enable auto-update or check for new releases regularly

## Acknowledgments

We appreciate the security research community's contributions. Hall of fame:

- (To be populated with security researcher names)