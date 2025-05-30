Here's your updated **`testing.policy.md`** file, with the requested additions about unit and E2E tests for backend endpoints — clearly specifying test coverage for both success and error cases, and daily E2E tests for Kubernetes-related endpoints:

---

````md
# 🧪 Testing Policy

## 🖥️ Frontend (React)

### ✅ Unit Tests

* Run all unit tests:

  ```bash
  pnpm vitest run
````

* Run unit tests in watch mode:

  ```bash
  pnpm vitest
  ```

* Generate coverage report (optional):

  ```bash
  pnpm vitest run --coverage
  ```

* ⚠️ Coverage threshold for **unit tests** is **not enforced**, only logic/business logic functions must be tested.

### 🔌 Integration Tests

* Run integration tests using Playwright:

  ```bash
  pnpm playwright test --project=integration
  ```

* Debug integration tests:

  ```bash
  pnpm playwright test --project=integration --debug
  ```

### 🧭 End-to-End Tests (E2E)

* Run all E2E tests:

  ```bash
  pnpm playwright test --project=e2e
  ```

* Open Playwright UI (visual mode):

  ```bash
  pnpm playwright test --ui
  ```

* Run E2E tests on staging:

  ```bash
  BASE_URL=https://staging.example.com pnpm playwright test --project=e2e
  ```

* ✅ **E2E tests are required** to cover all critical paths (login, signup, booking, etc.)

* 🎯 **Coverage is enforced only for E2E tests**, not for unit/integration

### 🚦 Performance Tests

* Integrated with `web-vitals` (already in app)
* Use browser DevTools → Performance tab
* Run Lighthouse locally:

  ```bash
  npx lighthouse http://localhost:3000
  ```

### 🔐 Security

* Automatic checks via GitHub **Dependabot**

* Manual audit:

  ```bash
  pnpm audit
  ```

* Only custom-written code is tested — dependencies like Axios or framework utilities are not re-tested.

---

## 🦀 Backend (Rust)

### 🧪 Endpoint Tests

* All API endpoints are tested using `reqwest` + `tokio::test`
* Each endpoint includes **unit tests** for both **success and error cases**
* These unit tests are used to validate expected responses, error handling, and status codes

#### ✅ Run Backend Tests:

```bash
cd backend
cargo test
```

### 🧭 End-to-End (E2E) Tests

* E2E tests are also defined for endpoints interacting with **Kubernetes**
* These tests cannot be fully mocked/unit-tested and require a live cluster or mock API server
* ✅ These E2E tests are scheduled to run **daily** to ensure cluster-related functionality remains valid

#### 📅 Schedules

```yaml
on:
  schedule:
    - cron: '0 3 * * 1'   # Weekly endpoint unit tests (every Monday 03:00 UTC)
    - cron: '0 2 * * *'   # Daily E2E tests for Kubernetes-related endpoints (02:00 UTC)
```

---

## 🚀 CI/CD Enforcement

* ✅ All tests must pass before merging
* ✅ All E2E tests must pass before deployment to production
* 🧪 A weekly scheduled job runs backend endpoint unit tests
* 🧪 A daily scheduled job runs Kubernetes-related E2E tests

---

## 📝 Notes

* ✅ Use mocks for unit tests
* ✅ Use real components/services for integration tests
* ❌ Do not test third-party libraries (e.g., Axios, Tailwind, etc.)
* ✅ Test only custom logic and business-critical parts
* ✅ Backend endpoints must have unit tests for both success and failure scenarios
* ✅ Kubernetes-dependent backend features are E2E tested daily
