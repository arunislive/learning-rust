# Multithreaded Blockchain in Rust

This project implements a **multithreaded blockchain** in Rust using **proof-of-work mining**. It demonstrates Rust's concurrency model, memory safety features, and cryptographic hashing.

---

## 🚀 Features
- **Blockchain with Proof-of-Work Mining**
- **Multithreading for Parallel Mining**
- **Safe Memory Access (Arc, Mutex)**
- **SHA-256 Hashing for Block Security**

---

## 🛠 How It Works

### **1. Blockchain Structure**
- Uses `Mutex<Vec<Block>>` to store the blockchain safely.
- Allows multiple threads to mine new blocks concurrently.

### **2. Proof-of-Work Mining**
- Uses **SHA-256 hashing**.
- Requires the hash to start with **"0000"** (like real mining).
- The miner repeatedly hashes with a changing `nonce` value until the condition is met.

### **3. Concurrency with Threads**
- Uses **5 parallel threads** to mine blocks.
- **Arc (Atomic Reference Counting)** allows shared ownership of the blockchain.
- **Mutex** ensures thread-safe access to blockchain data.

---

## 📌 Sample Output
```plaintext
Block {
    index: 0,
    timestamp: 1700000000,
    data: "Genesis Block",
    prev_hash: "00000000000000000000000000000000",
    hash: "00000000000000000000000000000000",
    nonce: 0
}
Block {
    index: 1,
    timestamp: 1700000005,
    data: "Block 1",
    prev_hash: "0000345a6fb1c2...",
    hash: "00004d9e3f1a5b...",
    nonce: 10345
}
Block {
    index: 2,
    timestamp: 1700000010,
    data: "Block 2",
    prev_hash: "00004d9e3f1a5b...",
    hash: "000085d2f78c1a...",
    nonce: 20897
}
```

---

## 📜 Rust Code Overview

### **1. Define a `Block` Structure**
```rust
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    prev_hash: String,
    hash: String,
    nonce: u64,
}
```

### **2. Blockchain Implementation with Thread-Safety**
```rust
struct Blockchain {
    chain: Mutex<Vec<Block>>,
}
```

### **3. Proof-of-Work Hashing Function**
```rust
fn hash_block(index: u64, timestamp: u64, data: &str, prev_hash: &str, nonce: u64) -> String {
    let input = format!("{}{}{}{}{}", index, timestamp, data, prev_hash, nonce);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}
```

### **4. Mining Function**
```rust
fn mine_block(prev_block: Block, data: String) -> Block {
    let index = prev_block.index + 1;
    let timestamp = now();
    let prev_hash = prev_block.hash.clone();
    let mut nonce = 0;

    loop {
        let hash = hash_block(index, timestamp, &data, &prev_hash, nonce);
        if &hash[..4] == "0000" {
            return Block { index, timestamp, data, prev_hash, hash, nonce };
        }
        nonce += 1;
    }
}
```

### **5. Multithreaded Execution**
```rust
fn main() {
    let blockchain = Arc::new(Blockchain::new());
    let mut handles = vec![];

    for i in 0..5 {
        let blockchain_clone = Arc::clone(&blockchain);
        let handle = thread::spawn(move || {
            let data = format!("Block {}", i + 1);
            blockchain_clone.add_block(data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    blockchain.print_chain();
}
```

---

## 🏆 Why This Is Awesome
✅ **Fully functional blockchain**
✅ **Multithreaded mining simulation**
✅ **Rust memory safety: No race conditions!**
✅ **No external database required**

This serves as an advanced Rust exercise in multithreading, cryptography, and ownership principles! 🚀

