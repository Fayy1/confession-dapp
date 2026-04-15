# 🕊️ Confession Board dApp

A decentralized application (dApp) that allows users to post anonymous messages on-chain and interact with them through likes.

Built using **Rust** and **Soroban Smart Contracts** on the **Stellar Testnet**.

---

## 🚀 Features

* 📝 Post anonymous messages
* 📖 View all messages stored on-chain
* ❤️ Like / upvote messages
* 💾 Fully decentralized (data stored on-chain)

---

## 🛠️ Tech Stack

* Rust (Smart Contract)
* Soroban SDK
* Stellar Testnet

---

## 📁 Project Structure

```
stellar-workshop-starter/
│── contracts/
│   └── confession/
│       ├── src/
│       │   └── lib.rs
│       ├── Cargo.toml
│       └── test.rs
│
│── README.md
```

---

## ⚙️ Setup

### 1. Clone Repository

```bash
git clone https://github.com/Fayy1/confession-dapp.git
cd confession-dapp/stellar-workshop-starter
```

---

### 2. Install Requirements

Make sure you have installed:

* Rust → https://www.rust-lang.org/
* Stellar CLI

---

### 3. Navigate to Contract

```bash
cd contracts/confession
```

---

## 🔨 Build Contract

```bash
stellar contract build
```

---

## 🚀 Deploy to Testnet

```bash
stellar network use testnet
stellar contract deploy
```

Save the **contract ID**.

---

## ▶️ Usage

### 📝 Post Message

```bash
stellar contract invoke \
  --id <contract_id> \
  --fn post_message \
  --arg content="Hello Web3!"
```

---

### 📖 Get Messages

```bash
stellar contract invoke \
  --id <contract_id> \
  --fn get_messages
```

---

### ❤️ Like Message

```bash
stellar contract invoke \
  --id <contract_id> \
  --fn like_message \
  --arg id=12345
```

---

## 📖 How It Works

1. Users submit anonymous messages
2. Messages are stored on-chain using Soroban
3. Each message contains:

   * Unique ID
   * Message content
   * Number of likes
4. Other users can like messages

---

## 🌐 Network

This project is deployed on the **Stellar Testnet**.

---

## 🔮 Future Improvements

* 👤 Wallet-based identity (multi-user support)
* 🔥 Trending messages system
* 🚫 Spam prevention
* 🌐 Frontend UI (React / Flutter)

---

## 👨‍💻 Author

Fayy1