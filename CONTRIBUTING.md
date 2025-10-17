# Contributing to Mainframe

Welcome to the Mainframe protocol! We're excited to have you contribute to building the future of agent-linked NFTs on Solana.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Code Standards](#code-standards)
- [Security Guidelines](#security-guidelines)
- [Testing Requirements](#testing-requirements)
- [Pull Request Process](#pull-request-process)
- [Community & Support](#community--support)

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

### Prerequisites

Before contributing, ensure you have:

- **Rust** (1.81+)
- **Solana CLI** (1.18.26+)
- **Anchor CLI** (0.31.1)
- **Node.js** (18+)
- **Yarn** package manager
- **Git** for version control

### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/maikers-mainframe
   cd maikers-mainframe
   ```

2. **Install Dependencies**
   ```bash
   # Install Rust dependencies
   cargo --version
   
   # Install Anchor CLI
   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
   avm install 0.31.1
   avm use 0.31.1
   
   # Install JavaScript dependencies
   yarn install
   ```

3. **Build the Program**
   ```bash
   anchor build
   ```

4. **Run Tests**
   ```bash
   anchor test
   ```

If all tests pass, you're ready to start contributing!

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
```

### 2. Make Your Changes

- Write code following our [Code Standards](#code-standards)
- Add tests for new functionality
- Update documentation as needed
- Ensure security considerations are addressed

### 3. Test Your Changes

```bash
# Run all tests
anchor test

# Run specific test
anchor test --grep "test_name"

# Check formatting
cargo fmt --all -- --check

# Run linting
cargo clippy --all-targets --all-features -- -D warnings

# Security audit
cargo audit
```

### 4. Commit Your Changes

Use clear, descriptive commit messages:

```bash
git add .
git commit -m "feat: add agent batch creation instruction

- Implement batch_create_agents instruction
- Add comprehensive tests for batch operations
- Update documentation for new functionality
- Optimize compute units for multiple agents"
```

### 5. Submit a Pull Request

1. Push your branch to GitHub
2. Create a pull request with a clear title and description
3. Link any related issues
4. Wait for code review and address feedback

## Code Standards

### Rust Code

- **Formatting**: Use `rustfmt` with default settings
- **Linting**: Follow `clippy` recommendations
- **Documentation**: Document all public functions and modules
- **Error Handling**: Use proper error types and messages
- **Security**: Follow Solana security best practices

### TypeScript Code

- **Formatting**: Use project's ESLint/Prettier configuration
- **Types**: Use strict TypeScript with proper type annotations
- **Testing**: Write comprehensive tests for client code
- **Documentation**: Include JSDoc for public APIs

### Documentation

- **README**: Keep it current and accurate
- **Code Comments**: Explain complex logic and security considerations
- **Architecture Docs**: Update when making structural changes
- **API Documentation**: Document all public interfaces

## Security Guidelines

Security is paramount when dealing with financial transactions on Solana.

### Critical Security Practices

1. **Input Validation**
   - Validate all user inputs in program instructions
   - Check account ownership and permissions
   - Verify PDA derivations

2. **Access Controls**
   - Implement proper authorization checks
   - Use account constraints appropriately
   - Follow principle of least privilege

3. **Economic Security**
   - Verify fee calculations thoroughly
   - Test edge cases in fee distribution
   - Ensure no arithmetic overflow/underflow

4. **Testing**
   - Write security-focused test cases
   - Test malicious input scenarios
   - Verify account state transitions

### Never Commit

- Private keys or mnemonics
- API keys or secrets
- Hardcoded addresses (use constants)
- Test tokens or mainnet data

### Security Review Process

All security-sensitive changes must:

1. Include comprehensive tests
2. Be reviewed by security-focused maintainers
3. Pass security audit checks
4. Be documented with security implications

## Testing Requirements

### Program Tests

- **Unit Tests**: Test individual functions and edge cases
- **Integration Tests**: Test complete instruction flows
- **Security Tests**: Test malicious scenarios and access controls
- **Performance Tests**: Verify compute unit usage

### Test Coverage

- All new instructions must have comprehensive tests
- Edge cases and error conditions must be tested
- Security-sensitive code requires additional test coverage
- Fee calculations need thorough validation

### Running Tests

```bash
# All tests
anchor test

# With coverage (if available)
anchor test --coverage

# Security audit
cargo audit

# Performance profiling
anchor test --profile
```

## Pull Request Process

### Before Submitting

1. **Self Review**: Review your own code thoroughly
2. **Testing**: Ensure all tests pass
3. **Documentation**: Update relevant documentation
4. **Security**: Consider security implications
5. **Performance**: Check for performance regressions

### PR Requirements

- [ ] Clear, descriptive title
- [ ] Detailed description of changes
- [ ] All tests passing
- [ ] No breaking changes (unless justified)
- [ ] Documentation updated
- [ ] Security considerations addressed
- [ ] Performance impact assessed

### Review Process

1. **Automated Checks**: CI/CD pipeline must pass
2. **Code Review**: At least one maintainer approval
3. **Security Review**: Security-sensitive changes need security team approval
4. **Final Approval**: Core team member final approval

### After Approval

- Squash commits if requested
- Merge to main branch
- Monitor for any issues post-merge

## Community & Support

### Getting Help

- **Documentation**: Check [docs/](docs/) directory
- **Issues**: Search existing GitHub issues
- **Discussions**: Use GitHub Discussions for questions
- **Security**: Email security@maikers.com for security issues

### Communication Channels

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Technical questions and community discussions
- **Pull Requests**: Code contributions and reviews

### Contributing Areas

We welcome contributions in these areas:

- **Core Protocol**: Anchor program improvements
- **SDK Development**: Client library enhancements  
- **Documentation**: Guides, tutorials, and reference docs
- **Testing**: Test coverage and infrastructure
- **Security**: Security audits and improvements
- **Performance**: Optimization and monitoring
- **Developer Tools**: Build tools and development aids

## Recognition

We value all contributions! Contributors may be recognized through:

- GitHub contributor status
- Mention in release notes
- Community shoutouts
- Potential collaboration opportunities

## License

By contributing to Mainframe, you agree that your contributions will be licensed under the [Apache License 2.0](LICENSE).

---

**Ready to contribute?** Start by checking our [good first issues](https://github.com/maikershq/maikers-mainframe/labels/good%20first%20issue) or reach out in GitHub Discussions!

**Thank you for helping build the future of agent-linked NFTs! 🤖✨**
