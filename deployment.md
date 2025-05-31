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
* Uses Node.js 18 + pnpm 10

#### `build-backend-docker`

* Builds the Rust backend as a Docker image using multi-stage builds
* Uses `cargo-chef` for efficient caching

#### Sample Workflow: `.github/workflows/ci.yml`

```yaml
name: CI Build and Docker

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

jobs:
  build-frontend:
    name: 🛠️ Build Frontend
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: ./.github/actions/setup-env
      - run: |
          cd frontend
          pnpm install
          pnpm build

  build-backend-docker:
    name: 🐳 Build Backend Docker Image
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: docker/setup-buildx-action@v3
      - run: |
          docker build -f backend/Dockerfile -t my-backend:latest .
```

---

## 💪 Local Development

### 💻 Option 1: Run Locally (Without Docker)

#### Prerequisites

* Rust (`rustup`)
* Node.js + pnpm (`corepack enable && corepack prepare pnpm@latest`)
* Postgres running locally (or via Docker)

#### ✅ Run Backend

```bash
cd backend
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
cargo run --bin backend
```

#### ✅ Run Frontend

```bash
cd frontend
pnpm install
pnpm dev
```

---

### 🐳 Option 2: Run With Docker

#### 🐘 Run Postgres with Docker

```bash
docker run --name local-postgres \
  -e POSTGRES_PASSWORD=postgres \
  -p 5432:5432 \
  -d postgres
```

#### ⚖️ Build Backend Docker Image

```bash
docker build -f backend/Dockerfile -t my-backend:local .
```

#### ▶️ Run Backend in Docker

```bash
docker run -p 8080:8080 \
  -e DATABASE_URL=postgres://postgres:postgres@host.docker.internal:5432/postgres \
  my-backend:local
```

#### 🌐 Run Frontend Locally

Still run the frontend locally for fastest iteration:

```bash
cd frontend
pnpm dev
```

Make sure API calls point to `http://localhost:8080`.

---

## 🧱 Optional: dockerfile.anvil

To build and run the `anvil` test chain locally:

```bash
docker build -f dockerfile.anvil -t anvil:local .
docker run -p 8545:8545 anvil:local
```

---

## 📌 Notes

* The `pnpm-lock.yaml` is located in `frontend/`. All `pnpm` commands must be run from there.
* `cargo` builds require `DATABASE_URL` to be set correctly, even for local usage.
* `host.docker.internal` allows Docker containers to access the host machine DB (use `localhost` if not inside Docker).

---
