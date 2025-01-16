# 🦀 Blog Backend API (Rust + PostgreSQL)

This repository contains the backend API for a modern blog platform. Built using **Rust**, **Axum**, and **PostgreSQL**, it provides a robust and scalable solution for handling blog data, authentication, and API endpoints. The backend is designed to integrate seamlessly with a **SvelteKit frontend** and is deployed using **Shuttle**.

---

## 🚀 Features
- **RESTful API** for managing blog posts, users, and comments.
- **PostgreSQL** as the primary database, hosted on **Supabase**.
- **Secure Authentication** with token-based user management (e.g., JWT).
- **Extensible and Modular** codebase using Rust best practices.
- **Cloud Deployment** using **Shuttle** for production-ready hosting.

---

## 🛠️ Tech Stack
- **Rust**: Safe, fast, and memory-efficient backend language.
- **Axum**: A lightweight and ergonomic web framework for Rust.
- **PostgreSQL**: A powerful relational database for structured data.
- **Supabase**: Database hosting with built-in API support.
- **Shuttle**: Cloud-native deployment for Rust applications.
- **SQLx**: Async, compile-time-checked SQL queries.

---

## 📂 Project Structure
```plaintext
src/
├── main.rs           # Entry point for the application
├── routes/           # Handlers and routes for API endpoints
│   ├── mod.rs        # Module definitions for routes
│   ├── posts.rs      # Routes for managing blog posts
│   ├── users.rs      # Routes for user authentication
├── db.rs             # Database connection setup
├── models.rs         # Structs and types for database models
├── errors.rs         # Custom error handling
migrations/           # SQL migrations for the PostgreSQL database
.env                  # Environment variables (e.g., DATABASE_URL)
```
---

## 📋 API Endpoints
| Method | Endpoint       | Description               |
|--------|----------------|---------------------------|
| GET    | `/posts`       | Fetch all blog posts      |
| POST   | `/posts`       | Create a new blog post    |
| GET    | `/posts/:id`   | Fetch a specific post     |
| PUT    | `/posts/:id`   | Update a blog post        |
| DELETE | `/posts/:id`   | Delete a blog post        |
| POST   | `/users/login` | User authentication       |
| POST   | `/users/signup`| User registration         |

---

## ⚙️ Installation and Setup

### Prerequisites
- Rust and Cargo installed: [Get Rust](https://www.rust-lang.org/tools/install)
- PostgreSQL instance (local or hosted on Supabase).

### Steps
1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/blog-backend-api.git
   cd blog-backend-api
2. Set up environment variables: Create a .env file with the following:
3. Run database migrations:
4. Run the development server:

---

## 🌐 Deployment
This backend is designed to be deployed using **Shuttle**:
1. Install Shuttle:
   ```bash
   cargo install cargo-shuttle
3. Deploy the app:
   ```bash
   cargo shuttle deploy

---

## 🤝 Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.



