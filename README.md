# 🦀 Blog Backend API (Rust + PostgreSQL)

This repository contains the foundational code for a backend API built using **Rust**, **Axum**, and **PostgreSQL**. Currently, the project focuses on establishing a simple connection to a PostgreSQL database and validating the setup with a basic ping query. The backend is in its initial stages of development and serves as a starting point for more complex functionality in the future.

---

## 🚀 Current Features
- **Database Connectivity**: Establishes a connection to a PostgreSQL database using `SQLx`.
- **Basic Query Execution**: Runs a test query (`SELECT 1`) to verify database connectivity.

---

## 🛠️ Tech Stack
- **Rust**: The programming language used for backend development.
- **Axum**: Web framework for building the API (planned but not yet implemented).
- **PostgreSQL**: The database used for persistent storage.
- **SQLx**: Async library for compile-time-checked SQL queries.
- **Docker**: Used for local development environment setup.

---

## 📂 Project Structure
```plaintext
.env                  # Environment variables (e.g., DATABASE_URL)
.devcontainer/        # Configuration for GitHub Codespaces
src/
├── main.rs           # Entry point for the application
Cargo.toml            # Rust project configuration
docker-compose.yml    # Docker Compose file for setting up services
LICENSE               # License file for the project
README.md             # Project documentation
```