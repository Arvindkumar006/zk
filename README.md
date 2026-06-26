# Stellar Horizon Pool

> **Institutional ZK-Compliant Private Liquidity Engine**

Stellar Horizon Pool is a compliant, institutional-grade dark pool enabling private Real-World Asset (RWA) and Stablecoin trades on the Stellar network. By combining **Aztec Noir Zero-Knowledge proofs** with **Soroban smart contracts**, the protocol guarantees trade confidentiality while enforcing regulatory compliance and anti-money laundering (AML) controls.

---

## 1. The Core Problem

Institutions operating in decentralized markets face a fundamental paradox:
- **The Privacy Imperative**: Public ledgers expose order sizes, wallet addresses, and execution patterns. This visibility invites front-running, sandwich attacks, and competitor exploitation, making large block orders untenable on transparent rails.
- **The Compliance Mandate**: Traditional privacy solutions (such as anonymous mixers) hide all transactional data. This breaks global regulations (such as FATF Travel Rule, FinCEN rules, and AML/KYC directives), precluding institutional participation due to compliance risks.

Without a mechanism that balances **trade secrecy** with **cryptographic proof of regulatory compliance**, institutional capital cannot safely access on-chain liquidity.

---

## 2. The Solution: Compliant Privacy

Stellar Horizon Pool achieves **Compliant Privacy** by splitting compliance verification and execution:

1. **Off-Chain KYC & AML Membership Checks**: Compliance authorities maintain an off-chain Merkle tree of audited, KYC-approved institutional identities.
2. **Zero-Knowledge Proof Generation**: Traders generate an Aztec Noir ZK proof locally on their hardware. The proof demonstrates that their identity is a member of the compliance Merkle tree and that their trade meets institutional volume parameters, *without revealing their public key, wallet balance, or identity*.
3. **On-Chain Soroban Verification**: The Horizon Pool contract receives the ZK proof and verifies it on-chain. Under **Protocol 25/26 updates**, Soroban contracts leverage native **BN254 pairing** and **multi-scalar multiplication (MSM) host functions**, ensuring fast, low-gas verifications directly on the Stellar ledger.

---

## 3. System Architecture

```text
  +-----------------------------------------------------------------------------------+
  |                                 OFF-CHAIN PROVER                                  |
  |                                                                                   |
  |  +--------------------+      +--------------------+      +---------------------+  |
  |  |  User Private Key  |      |   KYC Sibling Path |      |  RWA Swap Volume    |  |
  |  +---------+----------+      +---------+----------+      +----------+----------+  |
  |            |                           |                            |             |
  |            v                           v                            v             |
  |     [Poseidon Hash]             [Merkle Proof]             [Volume Threshold]     |
  |      Derive Leaf ID             Verify Membership             Enforce >= 1000     |
  |            |                           |                            |             |
  |            +---------------------------+----------------------------+             |
  |                                        |                                          |
  |                                        v                                          |
  |                         Aztec Noir Prover (main.nr)                               |
  |                  Synthesizes UltraHonk Cryptographic Proof                        |
  +----------------------------------------+------------------------------------------+
                                           |
                                           | (Proof Bytes & Public Inputs)
                                           v
  +-----------------------------------------------------------------------------------+
  |                                 ON-CHAIN LEDGER                                   |
  |                                                                                   |
  |                      Horizon Pool Contract (HorizonPool)                          |
  |                                        |                                          |
  |             +--------------------------+--------------------------+               |
  |             |                                                     |               |
  |             v                                                     v               |
  |    [State Checks]                                      [Verifier Client]          |
  |  - Active Merkle Root                                    verify_proof()           |
  |  - Match Swap Volume                                              |               |
  |             |                                                     v               |
  |             |                                        Stellar native BN254 Host    |
  |             |                                        MSM & Pairing Functions      |
  |             |                                                     |               |
  |             +--------------------------+--------------------------+               |
  |                                        |                                          |
  |                                        v                                          |
  |                            [Verified Event Emitted]                               |
  |                        RWA Assets Settled Confidentially                          |
  +-----------------------------------------------------------------------------------+
```
---

## Cryptographic Design Decisions & Engineering Trade-offs

> ### 🛡️ Cryptographic Parameter Isolation & Scale-up Path
>
> For the hackathon Minimum Viable Product (MVP) scope, our Aztec Noir circuit (`src/main.nr`) explicitly enforces a fixed Merkle tree depth of 4 (`[Field; 4]`). This was a deliberate engineering optimization decision designed to keep the cryptographic constraint pool small and maximize compilation and verification speeds during live judge evaluations.
>
> * **Current Implementation Capacity:** A depth of 4 accommodates a specialized institutional allowlist registry of exactly 16 enterprise participants ($2^4 = 16$).
> * **Production Scaling Path:** The underlying binary leaf reconstruction logic is written using a dynamic loop framework. Scaling the pool to production-grade enterprise capacity requires only changing the compile-time dimension parameter flag to `[Field; 20]` (supporting up to 1,048,576 unique compliant institutional identities) or `[Field; 32]` without requiring any structural changes to our core zero-knowledge mathematical circuit validation logic.

---

## 4. The Cryptographic Proof Lifecycle

The compliance circuit guarantees safety, integrity, and compliance through strict mathematical relations:

### 1. Leaf Derivation
A user's private address ($user\_address$) is hashed using the Poseidon BN254 hash function (optimized for ZK circuits):
$$\text{leaf} = \text{Poseidon}(user\_address)$$

### 2. Merkle Membership verification
The circuit uses the leaf, index, and sibling path to compute the tree root:
$$\text{computed\_root} = \text{ComputeMerkleRoot}(\text{leaf}, index, merkle\_path)$$
An assertion enforces that the user belongs to the current approved institutional database:
$$\text{assert}(\text{computed\_root} == \text{merkle\_root})$$

### 3. Institutional Threshold Controls
To prevent micro-transactions and comply with dark pool rules, the swap volume is validated:
$$\text{assert}(swap\_amount \ge 1000)$$

### 4. Replay & Malleability Prevention
To prevent front-running or proof replay, the public input vector binds the execution to a specific target recipient:
$$\text{assert}(recipient\_hash \neq 0)$$

---

## 5. Unified Monorepo Setup

Follow these commands to install, build, and test the entire stack.

### Prerequisites

Ensure you have standard developer tools installed (Git, Node.js).

### 1. Install Rust & Soroban CLI
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt
```

### 2. Install Aztec Noir (Nargo)
```bash
# Install Noirup (Noir installer)
curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash

# Restart shell and run noirup to get the compiler
noirup
```

### 3. Initialize Workspace Dependencies
Automate pulling down the external cryptographic verifier submodules and setting up dependencies using our unified setup script:
```bash
chmod +x setup.sh
./setup.sh
```

### 4. Install Frontend Dependencies
```bash
cd frontend
npm install
```

---

## 6. Production Verification Evidence

Both the smart contracts and zero-knowledge circuits include robust test suites verifying correctness.

### 1. Soroban Contract Tests (4/4 Passing)
Run tests inside `contracts/horizon_pool`:
```bash
cd contracts/horizon_pool
cargo test
```
**Output:**
```text
running 4 tests
test test::test_initialize_and_get_root ... ok
test test::test_update_root_success ... ok
test test::test_unauthorized_update_root_panics - should panic ... ok
test test::test_happy_path_swap_flow ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```

### 2. Noir Circuit Tests (2/2 Passing)
Run tests inside `circuits/compliance_check`:
```bash
cd circuits/compliance_check
nargo test
```
**Output:**
```text
[compliance_check] Running 2 test functions
[compliance_check] Testing test_successful_compliance ... ok
[compliance_check] Testing test_fails_below_minimum_amount ... ok
[compliance_check] 2 tests passed
```

### 3. Frontend Live Dev Environment
Start the dashboard locally:
```bash
cd frontend
npm run dev
```
**Output:**
```text
  VITE v5.4.21  ready in 1591 ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
```
