# OYTO

Programmable Compensation for Engineering Contributions

## Overview

OYTO is a Solana-based prototype that transforms engineering work into deterministic compensation events.

The project demonstrates how contribution-based compensation can be enforced through transparent, programmable rules rather than subjective decision-making.

## Problem

Compensation decisions in engineering organizations are often:

- Opaque
- Subjective
- Difficult to audit
- Difficult to justify

Contributors frequently lack visibility into how their work translates into rewards.

## Solution

OYTO introduces a programmable compensation engine where:

Engineering Contribution
↓
Verifiable Event
↓
Programmable Rule
↓
Compensation Event

Compensation outcomes become predictable, transparent, and auditable.

MVP Features
Rule Creation

Project owners can define compensation policies.

#### Example:

Merged Pull Request = 10 Units
Contribution Submission

Contributors submit work events.

The protocol evaluates the contribution and creates a compensation record.

### Anti-Gaming Enforcement

The protocol rejects contributions that fail predefined quality thresholds.

#### Example:

PR Size < 20

Results in:

_ContributionTooSmall_

## Smart Contract Architecture

- Instructions

```
create_rule
submit_contribution
```

- Accounts

```
RuleAccount
ContributionAccount
CompensationAccount
```

## Testing

The project includes Devnet integration tests covering:

### Create Rule

Validates creation of compensation rules.

---

### Generate Compensation

Validates deterministic reward calculation.

---

### Reject Invalid Contribution

Validates anti-gaming enforcement.

### Running Tests

Configure environment variables:

```bash
export ANCHOR_PROVIDER_URL=https://api.devnet.solana.com
export ANCHOR_WALLET=~/.config/solana/id.json
```

Run tests:

```bash
npm run test:devnet
```

#### Test output(Devnet)

Program ID: `AiD45wxFcNuh1QC1EAU5i3dxbyVGTyNnZUdiHPbh8Fvk`

-> [View on Solana Explorer](https://explorer.solana.com/address/AiD45wxFcNuh1QC1EAU5i3dxbyVGTyNnZUdiHPbh8Fvk?cluster=devnet)

##### Devnet Test Passing

![Devnet tests Passing](screenshots/test-3)

### Current Scope

This MVP intentionally excludes:

- GitHub integration
- Token transfers
- Treasury management
- Frontend application
- Contributor reputation systems

The objective is to validate the core hypothesis:

**Engineering contributions can be transformed into transparent, deterministic compensation events using programmable rules.**

## Future Roadmap

- GitHub webhook integration
- Contribution verification
- DAO treasury integration
- SPL token payouts
- Reputation scoring
- Open-source maintainer funding
- Contributor payroll automation

## Vision

| If work can be verified, compensation can be automated.
