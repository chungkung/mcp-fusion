## Description

<!-- Describe your changes in detail. What problem does this PR solve? -->

## Type of Change

- [ ] 🐛 Bug fix (non-breaking change that fixes an issue)
- [ ] ✨ New feature (non-breaking change that adds functionality)
- [ ] 💥 Breaking change (fix or feature that would cause existing functionality to change)
- [ ] 📚 Documentation update
- [ ] 🧪 Test coverage improvement
- [ ] 🔧 Refactoring / code cleanup
- [ ] ⚡ Performance improvement
- [ ] 🔒 Security fix

## Related Issues

<!-- Link to related issues using #issue_number -->

Closes #

## Checklist

- [ ] I have read the [CONTRIBUTING.md](CONTRIBUTING.md) guidelines
- [ ] My code follows the project's style guidelines
- [ ] I have run all checks locally:

  **Rust:**
  ```bash
  cd src-tauri
  cargo fmt --all -- --check
  cargo clippy --no-default-features -- -D warnings
  cargo test --no-default-features
  ```

  **Frontend:**
  ```bash
  cd src-frontend
  npx tsc --noEmit
  npx prettier --check "src/**/*.{ts,tsx,css,json}"
  npx vitest run
  ```

- [ ] I have added tests that prove my fix/feature works
- [ ] I have updated the documentation accordingly
- [ ] I have updated the CHANGELOG.md

## How Has This Been Tested?

<!-- Describe the tests you ran and how to reproduce them -->

## Screenshots (if applicable)

<!-- Add screenshots to help explain your changes -->