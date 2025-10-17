# Security Policy

## Reporting Security Vulnerabilities

The security of the Maikers Mainframe protocol is our top priority. We welcome and appreciate responsible disclosure of security vulnerabilities.

### How to Report

**Please do NOT create public GitHub issues for security vulnerabilities.**

Instead, please report security vulnerabilities through one of these secure channels:

- **Email**: [security@maikers.com](mailto:security@maikers.com)
- **Discord**: Join our server at [https://discord.gg/maikers](https://discord.gg/maikers) and open a support ticket
- **GitHub Security Advisories**: Use the "Report a vulnerability" button in the Security tab

### What to Include

When reporting a security vulnerability, please include:

1. **Description**: Clear description of the vulnerability
2. **Impact**: Potential impact and affected systems
3. **Reproduction**: Step-by-step instructions to reproduce the issue
4. **Proof of Concept**: Code, screenshots, or other evidence (if applicable)
5. **Suggested Fix**: Your recommendations for addressing the issue (optional)
6. **Contact**: Your preferred method for follow-up communication

### Response Timeline

We are committed to responding quickly to security reports:

- **Initial Response**: Within 24 hours of report
- **Assessment**: Within 72 hours we'll assess severity and validity
- **Updates**: Regular status updates every 5-7 days during investigation
- **Resolution**: Timeline depends on complexity, typically 30-90 days
- **Disclosure**: Coordinated disclosure after fix is deployed

### Scope

This security policy covers vulnerabilities in:

#### In Scope ✅
- **Mainframe Anchor Program**: Core Solana program logic
- **Agent Management**: Creation, transfer, pause, close operations
- **Fee System**: Calculation, collection, and distribution
- **Access Controls**: Authorization and permission validation
- **NFT Validation**: Ownership and metadata verification
- **Partner Collections**: Discount and verification systems
- **Related Infrastructure**: SDK, APIs, and integration points

#### Out of Scope ❌
- **Third-party Dependencies**: Issues in external libraries or services
- **Social Engineering**: Attacks targeting users rather than code
- **Physical Security**: Hardware or physical access attacks
- **DDoS Attacks**: Network-level denial of service
- **Already Known Issues**: Vulnerabilities we're already aware of

### Severity Classification

We use the following severity levels:

- **Critical**: Remote code execution, fund theft, protocol halt
- **High**: Unauthorized access, data corruption, fee manipulation
- **Medium**: Information disclosure, denial of service, logic errors
- **Low**: Minor information leaks, cosmetic issues

### Security Features

The Mainframe protocol implements multiple security layers:

- **Access Control**: NFT ownership verification and role-based permissions
- **Input Validation**: Comprehensive sanitization of all user inputs
- **Safe Arithmetic**: Protection against integer overflow/underflow
- **Atomic Operations**: Fee distribution and state updates are atomic
- **Emergency Controls**: Protocol pause mechanism for critical issues
- **Audit Trail**: Complete event logging for all operations

For detailed technical security information, see [docs/security.md](docs/security.md).

### Rewards and Recognition

While we don't currently offer a formal bug bounty program, we deeply appreciate security researchers who help us maintain a secure protocol:

- **Recognition**: Public acknowledgment (with your permission)
- **Communication**: Direct line to our security team
- **Updates**: Early notification of fixes and improvements
- **Collaboration**: Opportunity to work with our team on solutions

### Examples of Valid Reports

These types of issues would be considered valid security reports:

- **Smart Contract Vulnerabilities**: Logic errors, reentrancy, privilege escalation
- **Fee Manipulation**: Ways to bypass or manipulate fee calculations
- **Access Control Bypass**: Unauthorized operations or privilege escalation
- **Data Integrity**: Corruption of agent or protocol state
- **Economic Attacks**: MEV, sandwich attacks, or economic exploits

### Security Contacts

For security-related inquiries:

- **Primary**: [security@maikers.com](mailto:security@maikers.com)
- **Discord**: [https://discord.gg/maikers](https://discord.gg/maikers) (open a support ticket)
- **Direct Contact**: Contact `chainsona` if needed
- **GitHub**: [@MaikersHQ](https://github.com/MaikersHQ)

### Additional Resources

- **Program Documentation**: [docs/](docs/)
- **Technical Security Model**: [docs/security.md](docs/security.md)
- **Economics & Fees**: [docs/economics.md](docs/economics.md)
- **Source Code**: [programs/mainframe/](programs/mainframe/)
- **Tests**: [tests/mainframe.ts](tests/mainframe.ts)

### Security Updates

Security updates and patches will be:

1. **Tested** thoroughly on devnet before mainnet deployment
2. **Announced** via our official communication channels
3. **Documented** with clear upgrade instructions
4. **Coordinated** with major integrators and partners

### Legal

By reporting security vulnerabilities to us, you agree to:

- **Act in Good Faith**: No malicious activity or data destruction
- **Maintain Confidentiality**: Don't disclose until we've addressed the issue
- **Provide Reasonable Time**: Allow us to investigate and fix the issue
- **Follow Responsible Disclosure**: Work with us on disclosure timing

We commit to:

- **Not Pursue Legal Action**: Against good-faith security research
- **Work Collaboratively**: With researchers to understand and fix issues
- **Provide Credit**: Public recognition for your contribution (if desired)
- **Maintain Communication**: Regular updates throughout the process

---

**Thank you for helping keep Maikers Mainframe secure!**

*Last updated: October 2025*
