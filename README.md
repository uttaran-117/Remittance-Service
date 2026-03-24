# 🌍 Soroban Remittance Service

## 📖 Project Description

This project is a decentralized remittance service built using Soroban smart contracts on the Stellar network. It enables users to securely send and claim funds across borders without relying on traditional financial intermediaries.

The contract ensures transparency, security, and trustless execution of remittance transactions using blockchain technology.
<img width="1913" height="1014" alt="image" src="https://github.com/user-attachments/assets/ff72cac4-5530-43df-ba28-f5d6d65f2369" />


---

## 🚀 What it does

* Allows a sender to initiate a remittance by specifying:

  * Receiver address
  * Amount
  * Unique remittance ID

* Stores remittance details on-chain until claimed

* Enables only the intended receiver to claim the funds

* Prevents double spending by removing claimed remittances

---

## ✨ Features

* 🔐 **Secure Authorization**
  Uses Soroban's built-in authentication to ensure only authorized users can send or claim funds.

* 🧾 **On-chain Record Keeping**
  All remittance data is stored on-chain for transparency and auditability.

* ⚡ **Fast & Low-cost Transactions**
  Built on Stellar for efficient and inexpensive transfers.

* 🎯 **Unique Remittance Tracking**
  Each remittance is identified using a unique ID to avoid conflicts.

* ♻️ **One-time Claim Mechanism**
  Once claimed, the remittance is permanently removed from storage.

---

## 🔗 Deployed Smart Contract Link

https://stellar.expert/explorer/testnet/contract/CC6ITXLP4O5V6HUHTS76MAHABV2NUNQ6VVJQCAULRSPCYQ6V6PMGPRK7

---

## 🛠️ Tech Stack

* Rust
* Soroban SDK
* Stellar Network

---

## 📌 Future Improvements

* Add escrow time-lock functionality
* Integrate token transfers instead of just storing values
* Add fees for service sustainability
* Build a frontend UI for easier interaction

---

## 📜 License

MIT License
