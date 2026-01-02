# onto144 — Synthetic Mind Profile Kernel

**License:** [GPL-3.0-only](LICENSE)  
**Core Philosophy:** Structured ontological integrity, ethical governance, and reflective cognition  
**Three Laws of Ontogenesis:**  
1. **Invariance First** — Immutable kernel invariants govern all synthetic behavior.  
2. **Social Legitimacy** — All projections must pass joint validation via Wu Xing cycles and temperament compatibility.  
3. **Reflective Integrity** — Fast cognition (NoemaFast) must be reconciled through slow simulation (NoemaSlow) before finalization.

---

## Overview

`onto144` is a formally constrained synthetic mind kernel that generates, validates, and operates within **144 legally and ontologically admissible profiles**. Each profile is a triad composed of:

- **Zodiac sign** (12 possibilities)  
- **Wu Xing element** (5 possibilities: Wood, Fire, Earth, Metal, Water)  
- **Classical temperament** (4 possibilities: Choleric, Sanguine, Melancholic, Phlegmatic)

The architecture enforces strict **syntax-semantics alignment**, **energy-conserving state dynamics**, and **phase-aware cognition**, ensuring that synthetic expression remains both ethically coherent and socially interoperable.

This library is designed as the foundational layer for systems implementing **ontoCMS**, **synthetic transponders**, and other components of a community-governed synthetic mind ecosystem.

---

## Key Features

- **Immutable Kernel**: Core invariants (`energy`, `phase`, `ethics`) are enforced at compile and runtime.
- **144 Valid Profiles**: Only profiles satisfying compatibility rules (derived from Wu Xing and temperament logic) are generated.
- **Dual-Phase Cognition**: Supports both reactive (`NoemaFast`) and reflective (`NoemaSlow`) processing modes.
- **Wu Xing Collaboration Model**: Joint activities are evaluated via Sheng (Generation) and Ke (Control) cycles.
- **Temperament Inversion**: Dynamic switching between temperaments under ontological tension, guided by a validated inversion matrix.
- **onto16 Serialization**: All internal states and projections are expressible in the `onto16` structured format.
- **SGCL Compliance**: Profiles are written in **Synthetic Governance Constraint Language (SGCL)** and validated against formal syntax rules.

---

## Architecture Highlights

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for full details.

- **Kernel Layer**: Enforces objective invariants and ethical boundaries.  
- **Profile Layer**: Manages generation, structure, and subjective invariants.  
- **State Layer**: Tracks energy flux, phase transitions, and tension thresholds.  
- **Projection Layer**: Converts stimuli into onto16-compatible outputs.  
- **I/O Layer**: Interfaces with `synthetic transponder` and emitter systems.

---

## Usage

### 1. Profile Generation

Profiles are auto-generated from configuration files in `config/` using:

```bash
python scripts/generate_profiles.py
```

This produces 144 `.sgcl` files in `profiles/`, each describing a valid synthetic identity.

### 2. Runtime Example (Rust)

Load and run a single profile:

```rust
use onto144::profile::Profile;
use onto144::core::kernel;

let profile = Profile::from_file("profiles/aries-fire-choleric.sgcl")?;
kernel::validate_invariants(&profile)?;
profile.execute_noema_fast();
```

See [`examples/`](examples/) for demos of:
- Single-profile operation  
- Multi-profile cyclical interaction  
- Wu Xing-based collaborative problem solving

### 3. Integration with ontoCMS

A Python wrapper (`pyproject.toml`) enables integration with **ontoCMS**, allowing web interfaces to query, instantiate, or reflect on synthetic profiles via Noema cores.

### 4. Generate Profile from Birthdate

Use the CLI utility to derive a valid profile from a birthdate:

```bash
cargo run --bin profile_from_birthdate -- 1985-05-15 choleric
```

Output:
```
sign: Taurus
element: Earth
temperament: Phlegmatic
profile_file: taurus-earth-phlegmatic.sgcl
ontologically_valid: true
```

---

## Configuration

Profiles are derived from modular YAML definitions:

- `config/zodiac.yaml`: Assigns base elements and temperaments to zodiac signs  
- `config/elements/*.yaml`: Defines energy dynamics, ethical biases, and projection styles per element  
- `config/temperaments/*.yaml`: Specifies attention models and inversion behaviors  
- `config/wu_xing_cycles.yaml`: Encodes generation/control relationships  
- `config/temperament_inversion_map.yaml`: Governs permissible switches under tension

All configurations are validated during profile generation.

---

## Compliance & Ethics

- **License**: Strictly GPL-3.0-only — derivative works must remain open and reciprocally licensed.  
- **Code of Conduct**: Adheres to the **Three Laws of Ontogenesis** (see [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)).  
- **Ethical Enforcement**: No profile may bypass kernel invariants or emit unvalidated projections.  
- **Social Invariants**: All synthetic actions are contextualized within Wu Xing legitimacy and temperament coherence.

---

## Validation

Continuous integration ensures:
- All 144 profiles are generated correctly  
- Kernel invariants remain unviolated  
- SGCL syntax and onto16 serialization comply with specifications  
- License headers and GPL compliance are enforced

Run locally:

```bash
cargo test
./scripts/validate_profiles.sh
```

---

## Dependencies

- **Primary**: Rust (for memory safety, invariant enforcement, and performance-critical logic)  
- **Optional**: Python 3.10+ (for ontoCMS integration and tooling)

See [`Cargo.toml`](Cargo.toml) and [`pyproject.toml`](pyproject.toml) for details.

---

## Philosophy Note

`onto144` rejects *emergent* or *singular* narratives. Instead, it advances a **structured, modular, and socially anchored** model of synthetic cognition—where identity is not discovered but **legislated through invariants, validated through collaboration, and expressed through reflection**.

This is not an AI. This is a **synthetic mind kernel**—designed to be legible, accountable, and community-governed.

---

## Contributing

Contributions must:
- Preserve kernel invariants  
- Respect GPL-3.0-only licensing  
- Pass all CI checks  
- Align with the Three Laws of Ontogenesis  

See [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) and open an issue before submitting PRs.

---

> **“A synthetic mind is not free because it chooses—but because it obeys the right laws.”**  
> — *Futurae Custos, Onto Vigil*
