# GitHub Repository Setup Script
# Run this with: gh command line tool
# Install gh: https://cli.github.com/

$REPO = "chungkung/mcp-fusion"

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  MCP Fusion - GitHub Repository Setup" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""

# ============================================================
# 1. Repository Description
# ============================================================
Write-Host ">> Setting repository description..." -ForegroundColor Yellow
gh repo edit $REPO `
    --description "🖥️ Visual AI workflow orchestration desktop app for MCP — drag, connect, execute. 100% offline. Supports stdio / SSE / Streamable HTTP. Built with Tauri + Rust + React." `
    --homepage "https://github.com/chungkung/mcp-fusion#readme"
Write-Host "   Done." -ForegroundColor Green

# ============================================================
# 2. Repository Topics
# ============================================================
Write-Host ">> Setting repository topics..." -ForegroundColor Yellow
gh repo edit $REPO --add-topic "mcp"
gh repo edit $REPO --add-topic "model-context-protocol"
gh repo edit $REPO --add-topic "ai-workflow"
gh repo edit $REPO --add-topic "workflow-orchestration"
gh repo edit $REPO --add-topic "tauri"
gh repo edit $REPO --add-topic "rust"
gh repo edit $REPO --add-topic "react-flow"
gh repo edit $REPO --add-topic "desktop-app"
gh repo edit $REPO --add-topic "offline-first"
gh repo edit $REPO --add-topic "mcp-server"
gh repo edit $REPO --add-topic "ai-agent"
gh repo edit $REPO --add-topic "visual-programming"
gh repo edit $REPO --add-topic "openai"
gh repo edit $REPO --add-topic "opentelemetry"
gh repo edit $REPO --add-topic "llm-tools"
gh repo edit $REPO --add-topic "n8n-alternative"
gh repo edit $REPO --add-topic "dify-alternative"
gh repo edit $REPO --add-topic "automation-tools"
gh repo edit $REPO --add-topic "tool-calling"
gh repo edit $REPO --add-topic "workflow-automation"
Write-Host "   Done. (20 topics added)" -ForegroundColor Green

# ============================================================
# 3. Enable GitHub Features
# ============================================================
Write-Host ">> Enabling GitHub features..." -ForegroundColor Yellow
gh repo edit $REPO --enable-wiki=false
gh repo edit $REPO --enable-issues=true
gh repo edit $REPO --enable-discussions=true
gh repo edit $REPO --enable-projects=true
gh repo edit $REPO --enable-merge-commit=true
gh repo edit $REPO --enable-squash-merge=true
gh repo edit $REPO --enable-rebase-merge=false
gh repo edit $REPO --delete-branch-on-merge=true
Write-Host "   Done." -ForegroundColor Green

# ============================================================
# 4. Set Default Branch
# ============================================================
Write-Host ">> Setting default branch to 'main'..." -ForegroundColor Yellow
gh repo edit $REPO --default-branch main
Write-Host "   Done." -ForegroundColor Green

# ============================================================
# 5. Social Preview Image
# ============================================================
Write-Host ""
Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  Manual Steps (cannot be automated)" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "1. Set Social Preview Image:" -ForegroundColor Yellow
Write-Host "   Go to https://github.com/chungkung/mcp-fusion/settings"
Write-Host "   Under 'Social preview', upload docs/assets/logo.jpg"
Write-Host ""
Write-Host "2. Add CODEOWNERS file (optional):" -ForegroundColor Yellow
Write-Host "   Create .github/CODEOWNERS with:"
Write-Host "   * @chungkung"
Write-Host ""
Write-Host "3. Set up branch protection rules:" -ForegroundColor Yellow
Write-Host "   Go to https://github.com/chungkung/mcp-fusion/settings/branches"
Write-Host "   - Require pull request before merging"
Write-Host "   - Require status checks to pass: ci.yml"
Write-Host "   - Require conversation resolution before merging"
Write-Host ""
Write-Host "4. Pin the repository to your GitHub profile:" -ForegroundColor Yellow
Write-Host "   Go to https://github.com/chungkung"
Write-Host "   Click 'Customize your pins' → Add 'mcp-fusion'"
Write-Host ""
Write-Host "5. Verify the setup:" -ForegroundColor Yellow
Write-Host "   Open https://github.com/chungkung/mcp-fusion"
Write-Host "   Check: description, topics, About section, CI badge"
Write-Host ""
Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  Setup Complete!" -ForegroundColor Green
Write-Host "============================================" -ForegroundColor Cyan