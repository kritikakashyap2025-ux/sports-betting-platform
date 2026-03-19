# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.
# ⚽ Sports Betting Platform (Soroban Smart Contract)

## 📌 Project Description
Sports Betting Platform is a decentralized application built using Soroban on the Stellar blockchain. It enables users to place bets on sports matches in a transparent, trustless, and secure environment without relying on centralized betting providers.

---

## 🚀 What It Does
This smart contract allows:
- Creation of sports games (e.g., Team A vs Team B)
- Users to place bets predicting the winner
- Admin to resolve the game outcome
- Users to claim rewards if their prediction is correct

All logic is executed on-chain, ensuring fairness and eliminating manipulation.

---

## ✨ Features

### 🔗 Decentralized Betting
- No middlemen or centralized control
- Fully governed by smart contract logic

### 🔐 Secure Transactions
- User authentication required for every action
- Funds and bets are recorded on-chain

### 📊 Transparent Outcomes
- Game results are publicly verifiable
- Betting records cannot be altered

### ⚡ Instant Payout Logic
- Winners receive 2x of their bet amount
- Simple and predictable reward mechanism

### 🧩 Core Functionalities
- Create Game
- Place Bet
- Resolve Game
- Claim Rewards

---

## 🛠️ Tech Stack
- **Soroban SDK (Rust)**
- **Stellar Blockchain**
- On-chain storage using Soroban environment

---

## 🔄 Workflow

1. A game is created with two teams  
2. Users place bets predicting the winner  
3. Game is resolved by setting the result  
4. Winners claim their rewards  

---

## ⚠️ Limitations
- Fixed payout (2x)
- No real-world sports API integration
- No betting deadlines or restrictions
- No liquidity pool or odds calculation

---

## 🔮 Future Scope
- Oracle integration for real-time sports results
- Dynamic odds system
- Betting pools and multi-user reward distribution
- Frontend dApp (React + Stellar wallet)
- Token-based betting ecosystem

---

## 📦 Deployed Smart Contract Link
https://stellar.expert/explorer/testnet/contract/CCMSOVTUR7SXWH2EQMQ75FOVGCRXLW2UZAKFDPE5WM2NQS4PIT4NWBEL

---

![WhatsApp Image 2026-03-19 at 3 12 03 PM](https://github.com/user-attachments/assets/fb61811c-c8bf-451f-a021-05d49f066fb5)
