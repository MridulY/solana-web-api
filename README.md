# 📝 Solana To-Do List Web API

This is a **RESTful API** built using **Rust (Actix-Web)** that interacts with a Solana-based smart contract.  
It enables users to **create, update, delete, and fetch tasks stored on-chain** without requiring a centralized database.

🔹 **Blockchain**: Solana  
🔹 **Smart Contract Framework**: Anchor  
🔹 **Backend Framework**: Actix-Web  


## ✅ Features

1. **Blockchain-Powered Task Management**  
   - All tasks are **stored directly on the Solana blockchain**.

2. **CRUD Operations**  
   - Create, Read, Update, and Delete tasks via **simple API calls**.

3. **Secure Wallet Authentication**  
   - Uses **Solana CLI’s `id.json`** to sign transactions **instead of exposing private keys**.

4. **Task Ownership Verification**  
   - Only the **task creator** can update or delete the task.

5. **Optimized for Performance**  
   - **Async API** using **Tokio & Actix-Web**.


## 🔧 Setup Instructions

### 1️⃣ Install Dependencies
Make sure you have:
- **Rust** → `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Solana CLI** → `sh -c "$(curl -sSfL https://release.solana.com/stable/install)"`
- **Anchor Framework** → `cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked`

### 2️⃣ Set Up Solana Wallet
Generate a new wallet:
```sh
solana-keygen new --outfile ~/.config/solana/id.json
```

## **📌 4️⃣ API Endpoints**
1. Create a New Task
```
- Endpoint: `POST /api/todos`
- Body:
{
  "text": "Finish Rust project"
}
```
#### Response
```
{
  "id": "TaskPublicKey",
  "text": "Finish Rust project",
  "is_done": false,
  "created_at": 1741279648,
  "updated_at": 1741279648
}
```

2. Fetch a Task by ID
```
- Endpoint: GET /api/todos/{task_id}
```
Response
```
{
  "id": "TaskPublicKey",
  "text": "Finish Rust project",
  "is_done": false
}

```
3. Update a Task
```
- Endpoint: `PUT /api/todos/{task_id}`
- Body:
{
  "is_done": true
}
```
4. Delete a Task
```
- Endpoint: `DELETE /api/todos/{task_id}
```


---

## **🛠 6️⃣ Design Decisions**
```
## 🛠 Design Decisions

1. Actix-Web for High-Performance API
   - Uses asynchronous runtime to handle multiple requests efficiently.

2. Solana Smart Contract for Task Storage
   - Tasks are stored on-chain, eliminating the need for a database.

3. Secure Authentication Using Solana CLI
   - Instead of exposing private keys in requests, we load the wallet from `id.json`.

4. Fixed Storage Allocation (`Task::LEN = 1693` bytes)
   - Ensures efficient serialization and deserialization.

5. Task Ownership Verification
   - Only the original creator of a task can modify or delete it.

6. Stateless API for Efficient Execution
   - Each request interacts directly with Solana’s RPC, reducing overhead.


## 📌 Assumptions

1. Each User Has a Solana Wallet
   - Users must have a valid wallet address.

2. Users Must Have Enough SOL for Transactions
   - If an account does not have enough SOL, they need to airdrop funds: solana airdrop 2

3. Task IDs Are Public Keys
   - Each task is stored as a unique Solana account**.

4. Accounts Cannot Be Resized After Creation
   - If storage size is incorrect, the user must delete & recreate the task.

5. Transactions May Fail Due to Network Congestion**
   - If transactions fail, users should retry or check Solana logs: solana logs
```


