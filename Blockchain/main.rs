use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};
use std::thread;
use sha2::{Sha256, Digest};

// Define a Block structure
#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    prev_hash: String,
    hash: String,
    nonce: u64,
}

// Blockchain structure with a mutex for thread-safe access
struct Blockchain {
    chain: Mutex<Vec<Block>>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: now(),
            data: "Genesis Block".to_string(),
            prev_hash: "0".repeat(64),
            hash: "0".repeat(64),
            nonce: 0,
        };

        Blockchain {
            chain: Mutex::new(vec![genesis_block]),
        }
    }

    fn add_block(&self, data: String) {
        let last_block = self.chain.lock().unwrap().last().unwrap().clone();
        let new_block = mine_block(last_block, data);
        self.chain.lock().unwrap().push(new_block);
    }

    fn print_chain(&self) {
        let chain = self.chain.lock().unwrap();
        for block in chain.iter() {
            println!("{:#?}", block);
        }
    }
}

// Get current UNIX timestamp
fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

// Hash a block using SHA-256
fn hash_block(index: u64, timestamp: u64, data: &str, prev_hash: &str, nonce: u64) -> String {
    let input = format!("{}{}{}{}{}", index, timestamp, data, prev_hash, nonce);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

// Mining proof-of-work
fn mine_block(prev_block: Block, data: String) -> Block {
    let index = prev_block.index + 1;
    let timestamp = now();
    let prev_hash = prev_block.hash.clone();
    let mut nonce = 0;

    loop {
        let hash = hash_block(index, timestamp, &data, &prev_hash, nonce);
        if &hash[..4] == "0000" {
            return Block {
                index,
                timestamp,
                data,
                prev_hash,
                hash,
                nonce,
            };
        }
        nonce += 1;
    }
}

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
