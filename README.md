# 🦀 Rust Blockchain From Scratch

A minimalist blockchain runtime written in Rust, designed to explore core blockchain concepts like balances, system state, extrinsics, and proof-of-existence claims. Inspired by Substrate’s architecture.

---

## 🚀 Features

- ✅ Generic runtime using traits and macros
- ✅ Balance pallet with safe arithmetic and transfer logic
- ✅ System pallet with block number and nonce tracking
- ✅ Proof-of-Existence pallet (claim & revoke)
- ✅ Block execution with extrinsics
- ✅ Macro-based dispatch system: `#[call]`, `#[runtime]`

---

## 📁 Project Structure

src/ ├── balances.rs ├── system.rs ├── proof_of_existence.rs ├── main.rs ├── support.rs ├── types.rs ├── macros/ │ ├── call.rs │ └── runtime.rs


---

## 🛠️ Build Instructions

Ensure you have Rust and Cargo installed:

```bash
rustup update
cargo --version

📦 Install dependencies & compile:
```bash
cargo build

▶️ Run the project:
bash
cargo test


