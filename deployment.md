# 🚀 Deployment Guide

This guide describes how this repository is structured, how to run the app locally, and how the CI/CD pipeline is configured to **build, test, and deploy** the project using GitHub Actions.

---

## 📁 Repository Structure

The project is a monorepo containing:

```
.
📄 frontend/          # Vite + React app
│   ├── src/
│   ├── package.json
│   ├── pnpm-lock.yaml
│   └── ...
├── backend/           # Rust backend
│   ├── src/
│   └── ...
├── .github/
│   ├── workflows/
│   │   └── ci.yml         # GitHub Actions CI workflow
│   └── actions/
│       └── setup-env/     # Composite action to setup Node.js, Rust, pnpm
├── dockerfile.anvil      # Dockerfile for Anvil (optional)
```

---

## ⚙️ CI/CD Pipeline

### Triggered On:

* `push` and `pull_request` to `main` and `develop`

### Jobs

#### `build-frontend`

* Builds the frontend app in CI
* Uses Node.js 22 + pnpm 10

#### `build-backend-docker`

* Builds the Rust backend in CI

#### Sample Workflow: `.github/workflows/backend.yml`

```yaml
name: Backend CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    container: rust:latest

    steps:
      - uses: actions/checkout@v4

      - name: Build project
        working-directory: ./backend
        run: cargo build --release --locked --all-targets

  lint:
    runs-on: ubuntu-latest
    container: rust:latest

    steps:
      - uses: actions/checkout@v4

      - name: Install clippy
        run: rustup component add clippy

      - name: Run clippy
        working-directory: ./backend
        run: cargo clippy --locked --all-targets -- -D warnings

  format:
    runs-on: ubuntu-latest
    container: rustlang/rust:nightly

    steps:
      - uses: actions/checkout@v4

      - name: Install rustfmt
        run: rustup +nightly component add rustfmt

      - name: Check code formatting
        working-directory: ./backend
        run: cargo +nightly fmt -- --check
```
---

## 💻 Run Locally

#### Prerequisites

* Rust (`rustup`)
* Node.js + pnpm (`corepack enable && corepack prepare pnpm@latest`)
* Setup env variables

#### ✅ Run Backend

* Setup cube

```bash
cd backend
./setup.sh
```
* Start the backend

```bash
cargo run
```

#### ✅ Run Frontend

```bash
cd frontend
pnpm install
pnpm dev
```

---

