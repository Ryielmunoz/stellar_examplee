## Contract ID
CBSUXPFWQH323YM3DC7IVE4HFHFUE4EE2J4YJR36VCNMCE7WH5A5LKXV

!![Photos](stellarnigurz.png)


## Contract Link
https://stellar.expert/explorer/testnet/contract/CBSUXPFWQH323YM3DC7IVE4HFHFUE4EE2J4YJR36VCNMCE7WH5A5LKXV


# Stellar Notes DApp 📝🔒

A secure, tamper-proof, immutable personal note-taking engine built on the Stellar network using the Soroban Smart Contract SDK.

---

## 📌 Problem & Solution

### The Friction
Traditional cloud note-taking platforms rely on centralized databases. This compromises absolute data sovereignty, introduces privacy leak vectors, and leaves personal journals vulnerable to arbitrary platform deletion or server outages.

### The Solution
Stellar Notes moving productivity logging completely on-chain. Thoughts, credentials, or audit-logs are stored using zero-party assumptions inside isolated smart contract cells. Records remain cryptographically safe, completely permanent, and alterable exclusively by the verified private key owner.

---

## 🛠️ Stellar Features Used

* **Soroban Smart Contracts:** Drives logical CRUD mapping routines inside gas-optimized isolated environment boundaries.
* **Persistent Instance Storage:** Leverages state preservation tools on the native Stellar ledger layers for permanent data security.
* **Auth Verification SDK Hooks:** Guarantees that personal records can only be queried, written, or pruned by matching identity key signatures.

---

## 📋 Prerequisites

Ensure your local system has the following tools installed:
* **Rust Toolchain:** `rustc 1.74.0` or higher
* **Compilation Target:** `wasm32-unknown-unknown`
* **Soroban CLI Execution Layer:** Version `20.0.0` or higher

---

## ⚙️ Build & Test Commands

### 1. Build the Smart Contract
Compile the source code into an optimized WebAssembly (`.wasm`) payload ready for on-chain deployment:
```bash
soroban contract build
