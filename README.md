# Stellar Horizon Pool

An institutional, compliant dark pool for private RWA (Real-World Asset) and Stablecoin trades on the Stellar network using Zero-Knowledge proofs.

## Project Structure Blueprint

```text
stellar-horizon-pool/
├── .gitignore
├── README.md
├── circuits/
│   └── compliance_check/
│       ├── Nargo.toml
│       └── src/
│           └── main.nr
├── contracts/
│   └── horizon_pool/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
├── scripts/
│   └── deploy.ts (Future TypeScript deployment scripts)
└── lib/
    └── rs-soroban-ultrahonk/ (Noir verifier system dependency)
```

## Setup & External Dependencies

To clone and reference the recommended ZK verifier systems from the hackathon:

### 1. Noir Ultrahonk Verifier System (by yugocabrio)
This is used to verify Ultrahonk proofs generated from our Noir compliance circuits:
```bash
git clone https://github.com/yugocabrio/rs-soroban-ultrahonk.git lib/rs-soroban-ultrahonk
```

### 2. RISC Zero Verifier System (by Nethermind)
If we leverage RISC Zero for executing large off-chain computations:
```bash
git clone https://github.com/NethermindEth/stellar-risc0-verifier.git lib/stellar-risc0-verifier
```
