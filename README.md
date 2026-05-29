# Arc Wallet

A minimal Rust example for creating developer-controlled wallets on the Arc
network using Circle Wallets APIs.

This project was built as a learning and reference implementation for:

- Circle developer-controlled wallets
- Entity secret encryption flow
- Wallet set creation
- Wallet creation on `ARC-TESTNET`
- Async Rust backend architecture

Repository:

https://github.com/hkermanin/arc-wallet

---

# Features

- Async Rust architecture using `tokio`
- Shared `reqwest` HTTP client
- Circle API integration
- RSA-OAEP entity secret encryption
- Wallet set creation
- Developer-controlled wallet creation
- Typed request/response models with `serde`
- Environment-based configuration

---

# Tech Stack

- Rust
- Tokio
- Reqwest
- Serde
- RSA
- SHA256
- UUID

---

# Project Structure

```text
src/
├── config.rs
├── encrypt.rs
├── wallet_set.rs
├── wallet.rs
└── main.rs
```

---

# Prerequisites

Before running the project you need:

- A Circle Developer Account
- A Circle API Key
- A registered Entity Secret
- Rust installed

---

# Environment Variables

Create a `.env` file:

```env
CIRCLE_API_KEY=your_api_key
ENTITY_SECRET=your_entity_secret
```

---

# Install

```bash
git clone https://github.com/hkermanin/arc-wallet.git

cd arc-wallet

cargo build
```

---

# Run

```bash
cargo run
```

---

# What This Example Does

The application:

1. Loads Circle configuration
2. Fetches the Circle entity public key
3. Encrypts the entity secret
4. Creates a developer-controlled wallet set
5. Creates a wallet on `ARC-TESTNET`
6. Prints the wallet information

---

# Example Output

```text
wallet_set_id: c9c84f46-6106-5a1e-bb71-1ceb7368dd37

wallet_id: 1f29...

wallet_address: 0x1234...

blockchain: ARC-TESTNET
```

---

# Important Notes

- This project is intended as a minimal educational example.
- Do NOT expose your entity secret publicly.
- Store secrets securely in production environments.
- Wallet sets should typically be created once and persisted in a database.

---

# Future Improvements

- SQLite/PostgreSQL integration
- Wallet persistence
- Token transfers
- Balance queries
- Retry/error middleware
- Structured logging
- Modular SDK-style architecture

---

# References

- https://developers.circle.com/wallets
- https://developers.circle.com/api-reference/wallets/
- https://docs.arc.io/app-kit
