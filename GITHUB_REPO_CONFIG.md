# GitHub Repository Configuration Guide

## Repository Settings

### **Repository Description**
```
🤖 Permissionless Solana protocol linking AI agents to verified NFT collections via Anchor smart contracts with affiliate revenue sharing
```

### **Topics/Tags**
```
solana, anchor, nft, ai-agents, web3, defi, protocol, rust, typescript, 
blockchain, metaplex, revenue-sharing, affiliates, agent-nfts, agentic-economy
```

### **Repository Configuration**
```yaml
# Repository Settings
- Allow merge commits: ✅ (with title and description)
- Allow squash merging: ✅ (default)
- Allow rebase merging: ❌
- Automatically delete head branches: ✅
- Allow auto-merge: ✅
- Require conversation resolution: ✅

# Default branch: main
# Include administrators in branch protection: ✅
```

## Branch Protection Rules

### **Main Branch Protection**
```yaml
Branch name pattern: main
Restrictions:
  - Require a pull request before merging: ✅
  - Required approvals: 2
  - Dismiss stale reviews: ✅
  - Require review from CODEOWNERS: ✅
  - Require status checks to pass before merging: ✅
  - Require conversation resolution before merging: ✅
  - Include administrators: ✅
  - Allow force pushes: ❌
  - Allow deletions: ❌

Required status checks:
  - ci/anchor-build
  - ci/anchor-test  
  - ci/security-audit
  - ci/lint-check
```

## GitHub Actions Workflows

### **CI/CD Configuration**
```yaml
# .github/workflows/ci.yml
name: CI/CD Pipeline
on: [push, pull_request]

jobs:
  build-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install Anchor
        run: |
          npm install -g @coral-xyz/anchor-cli@0.31.1
      - name: Build program
        run: anchor build
      - name: Run tests
        run: anchor test
      - name: Security audit
        run: cargo audit

  security-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run Semgrep
        uses: returntocorp/semgrep-action@v1
```

## Issue & PR Templates

### **Bug Report Template**
```yaml
# .github/ISSUE_TEMPLATE/bug_report.yml
name: 🐛 Bug Report
description: Report a bug in Mainframe protocol
labels: ["bug", "needs-triage"]
body:
  - type: dropdown
    attributes:
      label: Component
      options:
        - Anchor Program
        - TypeScript Client
        - Documentation
        - SDK Integration
  - type: textarea
    attributes:
      label: Bug Description
      description: Clear description of the bug
    validations:
      required: true
  - type: textarea
    attributes:
      label: Reproduction Steps
      description: Steps to reproduce the behavior
    validations:
      required: true
  - type: input
    attributes:
      label: Anchor Version
      placeholder: "0.31.1"
    validations:
      required: true
```

### **Feature Request Template**
```yaml
# .github/ISSUE_TEMPLATE/feature_request.yml
name: ✨ Feature Request
description: Suggest new functionality for Mainframe
labels: ["enhancement", "needs-review"]
body:
  - type: dropdown
    attributes:
      label: Feature Category
      options:
        - Protocol Enhancement
        - SDK Improvement
        - Developer Experience
        - Documentation
        - Integration Support
```

### **Pull Request Template**
```markdown
# .github/pull_request_template.md
## Description
Brief description of changes

## Type of Change
- [ ] 🐛 Bug fix
- [ ] ✨ New feature  
- [ ] 📚 Documentation update
- [ ] 🔧 Refactoring
- [ ] ⚡ Performance improvement
- [ ] 🔒 Security enhancement

## Testing
- [ ] Anchor tests pass (`anchor test`)
- [ ] Integration tests updated
- [ ] Manual testing completed

## Security Considerations
- [ ] No new attack vectors introduced
- [ ] Fee calculations reviewed
- [ ] Access controls validated

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated (if needed)
- [ ] No breaking changes (or clearly documented)
```

## Repository Security

### **Security Policy**
```markdown
# SECURITY.md
# Security Policy

## Supported Versions
| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting Vulnerabilities
Report security vulnerabilities to: security@maikers.com

**DO NOT** create public GitHub issues for security vulnerabilities.

### Bug Bounty
We offer bounties for critical security findings:
- Critical: Up to $10,000 SOL
- High: Up to $5,000 SOL  
- Medium: Up to $1,000 SOL

### Audit Status
- [x] Internal security review
- [x] External security audit (pending)
- [x] Automated security scanning
```

### **Environment Variables & Secrets**
```yaml
# Required Repository Secrets
ANCHOR_PROVIDER_URL: # Solana RPC endpoint
ANCHOR_WALLET: # Base58 encoded keypair for testing
NPM_TOKEN: # For SDK publishing
SECURITY_CONTACT_EMAIL: security@maikers.com

# Environment Variables (for Actions)
RUST_VERSION: "stable"
NODE_VERSION: "18"
ANCHOR_VERSION: "0.31.1"
SOLANA_VERSION: "1.18.26"
```

## Community Health Files

### **Contributing Guidelines**
```markdown
# CONTRIBUTING.md
# Contributing to Mainframe

## Development Setup
1. Install Rust and Anchor CLI 0.31.1
2. Clone repository: `git clone https://github.com/maikershq/maikers-mainframe`
3. Build program: `anchor build`
4. Run tests: `anchor test`

## Code Standards
- Follow Rust best practices
- Use rustfmt for formatting
- Add comprehensive tests
- Document public APIs

## Security Guidelines
- Never commit private keys
- Validate all inputs in program instructions
- Follow Solana security best practices
- Get security review for major changes

## Pull Request Process
1. Fork repository
2. Create feature branch
3. Make changes with tests
4. Update documentation
5. Submit PR with clear description
```

### **Code of Conduct**
```markdown
# CODE_OF_CONDUCT.md
# Contributor Covenant Code of Conduct

## Our Pledge
We pledge to make participation in Mainframe development a harassment-free 
experience for everyone, regardless of background or identity.

## Standards
- Use welcoming and inclusive language
- Respect differing viewpoints and experiences  
- Accept constructive criticism gracefully
- Focus on community benefit
- Show empathy towards other community members

Contact: community@maikers.com for violations.
```

## Release Configuration

### **Release Workflow**
```yaml
# .github/workflows/release.yml
name: Release
on:
  push:
    tags: ['v*']

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build program
        run: anchor build
      - name: Create Release
        uses: actions/create-release@v1
        with:
          tag_name: ${{ github.ref }}
          release_name: Release ${{ github.ref }}
          draft: false
          prerelease: false
```

### **Changelog Configuration**
```yaml
# .github/release.yml
changelog:
  categories:
    - title: 🚀 Features
      labels: ["enhancement", "feature"]
    - title: 🐛 Bug Fixes  
      labels: ["bug", "fix"]
    - title: 🔒 Security
      labels: ["security"]
    - title: 📚 Documentation
      labels: ["documentation"]
    - title: ⚡ Performance
      labels: ["performance"]
```

## CODEOWNERS File

### **Code Review Assignment**
```
# .github/CODEOWNERS
# Global ownership
* @maikershq/core-team

# Anchor program
/programs/ @maikershq/protocol-team @maikershq/security-team

# Tests
/tests/ @maikershq/protocol-team

# Documentation
/docs/ @maikershq/docs-team

# Security-sensitive files
SECURITY.md @maikershq/security-team
/migrations/ @maikershq/protocol-team @maikershq/security-team
Cargo.toml @maikershq/protocol-team
Anchor.toml @maikershq/protocol-team
```

## Additional Recommendations

### **Repository Links**
```yaml
# Add to repository sidebar
Website: https://maikers.com
Documentation: https://docs.maikers.com/mainframe
SDK: https://github.com/maikershq/maikers-mainframe-sdk
Node Runtime: https://github.com/maikershq/maikers-mainframe-node
```

### **GitHub Pages**
Consider enabling GitHub Pages for:
- Program IDL documentation
- Integration examples
- Developer tutorials
- API reference

### **Project Board Configuration**
Create project boards for:
- 🚀 **Roadmap**: High-level feature planning
- 🐛 **Bug Triage**: Issue prioritization
- 🔄 **Active Development**: Current sprint work
- 📋 **Partner Requests**: Integration support

### **Labels Configuration**
```yaml
# Repository Labels
Type Labels:
  - bug (red)
  - enhancement (blue)
  - documentation (green)
  - security (purple)
  - performance (orange)

Priority Labels:
  - priority/critical (dark red)
  - priority/high (red)
  - priority/medium (yellow)
  - priority/low (green)

Status Labels:
  - status/needs-triage (gray)
  - status/in-progress (blue)
  - status/needs-review (orange)
  - status/blocked (red)

Component Labels:
  - component/anchor-program (purple)
  - component/typescript-client (blue)
  - component/documentation (green)
  - component/ci-cd (gray)
```

## Implementation Checklist

### **Phase 1: Repository Setup**
- [ ] Update repository description and topics
- [ ] Configure branch protection rules
- [ ] Set up required status checks
- [ ] Create CODEOWNERS file

### **Phase 2: Templates & Workflows**
- [ ] Add issue templates
- [ ] Add PR template
- [ ] Set up CI/CD workflow
- [ ] Configure security scanning

### **Phase 3: Community Health**
- [ ] Add CONTRIBUTING.md
- [ ] Add CODE_OF_CONDUCT.md
- [ ] Add SECURITY.md
- [ ] Configure release workflow

### **Phase 4: Project Management**
- [ ] Set up project boards
- [ ] Configure labels
- [ ] Set up repository secrets
- [ ] Enable GitHub Pages (if needed)

This configuration ensures the repository is professional, secure, and developer-friendly while maintaining the high standards expected for a protocol handling financial transactions on Solana.
