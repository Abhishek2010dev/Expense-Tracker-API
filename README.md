# 🏦 Expense Tracker API

An efficient and secure expense tracker API built using Rust with Axum, SQLx, and Tokio. This API allows users to manage their expenses, authenticate securely, and interact with a PostgreSQL database.

## ✨ Features

### 🔐 Authentication & Authorization
- **📝 User Registration**: Secure user signup with password hashing.
- **🔑 Login & Logout**: JSON Web Token (JWT)-based authentication.
- **♻️ Token Refresh**: Securely refresh access tokens.

### 💰 Expense Management
- **➕ Create Expense**: Add a new expense with category and amount validation.
- **📂 Retrieve Expenses**: Fetch user-specific expenses.
- **✏️ Update Expense**: Modify an existing expense.
- **🗑️ Delete Expense**: Remove an expense.
- **🔍 Filter by Category**: Fetch expenses by category.

### 🗄️ Database
- **🐘 PostgreSQL**: Used for persistent storage.
- **⚡ SQLx**: Asynchronous and compile-time safe query execution.

### 🚀 Caching
- **🛑 Redis**: Used for caching refresh tokens.

### 🔒 Security & Validation
- **🔐 Password Hashing**: Ensuring secure password storage.
- **🛡️ JWT Authentication**: Secure and scalable user authentication.
- **✅ Input Validation**: Enforces strict data validation.

### ⚡ Server & Performance
- **🌐 Axum**: A fast and lightweight framework for HTTP handling.
- **⚡ Tokio**: Ensures high concurrency and efficiency.
- **📦 Modular Architecture**: Well-structured modules for scalability.

## 📥 Installation

### 📌 Prerequisites
- 🦀 Rust (latest stable)
- 🐘 PostgreSQL
- 🛑 Redis

### ⚙️ Setup
```sh
# Clone the repository
git clone https://github.com/your-username/expense-tracker-api.git
cd expense-tracker-api

# Set up environment variables
cp .env.example .env

# Install dependencies
cargo build

# Run the database migrations
sqlx migrate run

# Start the server
cargo run
```

## 📌 API Endpoints

### 🔐 Authentication
- `POST /auth/register` - 📝 Register a new user
- `POST /auth/login` - 🔑 User login
- `POST /auth/logout` - 🚪 Logout
- `POST /auth/refresh` - ♻️ Refresh access token

### 💰 Expenses
- `POST /expenses` - ➕ Create a new expense
- `GET /expenses` - 📂 Retrieve all expenses
- `GET /expenses?category=` - 🔍 Filter expenses by category
- `PUT /expenses/{id}` - ✏️ Update an expense
- `DELETE /expenses/{id}` - 🗑️ Delete an expense

## 📜 License

MIT License. See [`LICENSE`](./LICENSE) for details.

## 📌 Reference
- [Expense Tracker API Roadmap](https://roadmap.sh/projects/expense-tracker-api)

