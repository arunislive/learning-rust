# Rust Execution Model and Key Concepts

This document serves as a structured guide to Rust’s execution model, memory management, and core concepts. It is designed to help you and others understand how Rust works under the hood.

---

# Compilation Process

Rust code goes through multiple stages during compilation:

1. **Lexical Analysis** – The source code is tokenized.
2. **Parsing & AST (Abstract Syntax Tree) Transformation** – Code is transformed into a tree representation.
3. **HIR & MIR (Intermediate Representations)** – Optimized versions of AST used for borrow checking and optimizations.
4. **LLVM IR (Low-Level Representation)** – Rust is translated to LLVM IR for further optimization.
5. **Code Generation (Machine Code)** – The final binary is produced.

## AST Transformation
- Expressions like `a + b` are represented as `BinaryOperation` in AST.
- Example AST for `let sum = 5 + 3;`:

  ```plaintext
  Expr::BinaryOperation {
      left: Expr::Literal(5),
      op: '+',
      right: Expr::Literal(3)
  }
  ```

---

# Memory Safety & Ownership

Rust enforces memory safety through ownership rules:

1. **Each value has one owner.**
2. **Values are dropped when the owner goes out of scope.**
3. **Borrowing Rules:**
   - Multiple immutable references (`&T`) are allowed.
   - Only one mutable reference (`&mut T`) is allowed at a time.

### Example: Borrowing Conflict
```rust
let mut s = String::from("hello");
let r1 = &s; // Immutable borrow
let r2 = &mut s; // ❌ ERROR: Cannot borrow `s` as mutable while it's also borrowed as immutable
```

---

# Panic Handling (`std::panic!`)

Rust has two types of errors:
1. **Recoverable Errors** – Handled using `Result<T, E>`.
2. **Unrecoverable Errors** – Handled using `panic!`.

### Example: Panic
```rust
fn main() {
    panic!("Something went wrong!");
}
```

### Catching Panics
```rust
use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        println!("Before panic");
        panic!("Oops!");
    });

    match result {
        Ok(_) => println!("No panic!"),
        Err(_) => println!("Caught a panic!"),
    }
}
```

---

# Threads (`std::thread`)

Rust provides a safe way to work with threads using `std::thread`.

### Example: Spawning Threads
```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..5 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(500));
    }

    handle.join().unwrap();
}
```

---

# Heap Memory Management

Rust provides smart pointers to manage heap memory efficiently.

### `Box<T>` – Heap Allocation
```rust
let b = Box::new(5);
println!("b = {}", b);
```

### `Rc<T>` – Reference Counting
```rust
use std::rc::Rc;

let a = Rc::new(String::from("hello"));
let b = Rc::clone(&a);
```

### `Arc<T>` – Thread-Safe Reference Counting
```rust
use std::sync::Arc;
use std::thread;

let a = Arc::new(5);
let a1 = Arc::clone(&a);
let handle = thread::spawn(move || {
    println!("{}", a1);
});
handle.join().unwrap();
```

---

# Conclusion

Rust’s execution model ensures:
- **Memory safety** without garbage collection.
- **Concurrency safety** without data races.
- **Efficient heap management** with smart pointers.

This document summarizes key concepts to help you review and share knowledge with others. 🚀

