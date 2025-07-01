# ✅ Task Manager CLI (Rust)

📋 A clean-architecture based command-line tool to manage your daily tasks — built with Rust, JSON storage

---

## ✨ Features

- ➕ Add tasks
- 📜 List all tasks
- ✅ Mark tasks as done
- 🗑️ Delete tasks
- 💾 Persists to `tasks.json` in local directory
- 🎨 Pretty colored CLI output
- 🧱 Clean modular structure using Rust best practices

---

## 🚀 Installation

```bash
git clone https://github.com/your-username/task-manager-cli.git
cd task-manager-cli
cargo build --release
```

## 🔧 Usage
All data is saved in tasks.json in the same folder.

### Add a Task
```bash

cargo run -- add "Read the Rust book"
```

### List All Tasks

```
cargo run -- list
```
### Sample Output

```
📝 Your Tasks:
 1. Read the Rust book       🕒 Pending  [2025-06-30 15:12]
 2. Watch RustConf 2025      ✅ Done     [2025-06-29 22:50]
```

### Mark Task as Done
```bash
cargo run -- done 1
```
Marks task number 1 as ✅ Done.

### Delete a Task
```
cargo run -- delete 2
```
Deletes task number 2.

## 🧪 Run Tests
```
cargo test
```

## 🧱 Project Structure

```
src/
├── adapter/           # CLI interface
│   ├── cli.rs
│   └── mod.rs
├── domain/            # Core model: Task, TaskStatus
│   ├── task.rs
│   └── mod.rs
├── usecase/           # Business logic: add, list, done, delete
│   ├── task_manager.rs
│   └── mod.rs
├── infrastructure/    # Storage: JSON read/write
│   ├── storage.rs
│   └── mod.rs
├── main.rs            # Entry point
└── lib.rs             # Exposes app as a library

```

## 🧠 Learnings (Optional Section)
This project demonstrates:

- Rust ownership, enums, structs, Result<T, E>

- File I/O with serde_json

- CLI building with clap

- Colored terminal output with colored

- Unit testing and clean architecture

