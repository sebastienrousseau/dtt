---
name: Feature Request
about: Propose a new feature, enhancement, or scoped piece of work
title: '[area] short, imperative title'
labels: feature
---

<!--
This template enforces the global convention for feature issues in
@sebastienrousseau repositories. Every section is required.

Reject (or ask the author to revise) any feature issue that:
  - has no user story,
  - has fewer than 5 acceptance criteria, or
  - has no target version.
-->

## User Story

**As a** [persona — e.g. "Rust developer building a financial service"]
**I want** [capability — e.g. "to convert datetimes between IANA timezones"]
**So that** [outcome — the actual benefit, not a restatement of the want]

## Acceptance Criteria

_Minimum **5** objectively verifiable criteria. Each must be in
**Given / When / Then** form and must be testable (i.e. a CI job or a
human reviewer can mark it pass/fail with no ambiguity)._

1. **Given** [precondition], **When** [action], **Then** [observable outcome].
2. **Given** [precondition], **When** [action], **Then** [observable outcome].
3. **Given** [precondition], **When** [action], **Then** [observable outcome].
4. **Given** [precondition], **When** [action], **Then** [observable outcome].
5. **Given** [precondition], **When** [action], **Then** [observable outcome].

## Technical Notes

_Constraints, design decisions, dependency considerations, alternatives
weighed, links to external specs. Keep this honest — including known
risks and trade-offs._

## Definition of Done

- [ ] Implementation (behind a feature flag if it adds optional deps)
- [ ] Unit tests covering every acceptance criterion
- [ ] 100% line + function coverage maintained on touched files
- [ ] 100% rustdoc coverage maintained (`cargo doc -D missing-docs`)
- [ ] Example in `examples/` demonstrating the feature end-to-end
- [ ] Benchmark in `benches/criterion.rs` (where applicable)
- [ ] CHANGELOG entry under the appropriate `###` heading
- [ ] Doctest in `README.md` if the feature is part of the public API
- [ ] MSRV unchanged, or bump documented in `docs/msrv-policy.md`
- [ ] All commits cryptographically signed (`git commit -S`)

## Target Release

**vX.Y.Z** (only increment by `0.0.1` for patches pre-1.0; bump the
minor digit when a feature changes the public API or carries a deliberate
MSRV move outside the security exception)
