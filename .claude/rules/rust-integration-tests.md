---
paths:
  - "**/tests/**/*.rs"
---

# Rust Integration Tests Standards

- Each test needs to be named after what functionality, or issue it actually tests.
- Each test file needs to be named after what functionality, or issue it actually tests.
- Each test represents a specific scenario that the core project needs to support, or represent an uncovered issue.
- If you uncover a new issue while testing, create yet another targeted test that covers that.
- Every test must use production code. Never recreate the original code to test something conceptually. Always use production code.

- They must be single-purpose.
- It must be clear what is being tested in the test file by just reading the filename.

# Clarification on when to create integration tests

- If a test can be expressed as a unit test, create a unit tests instead.
- Unit tests run faster than integration tests, so if a test does not validate combined functionalities, create a unit test instead.
- Prefer unit tests over integration tests; use integration tests to test actual integrations, or to validate that the combined components behave correctly when combined.
- Prefer integration tests when you need to interact with binaries, scripts, and such.
- Prefer integration tests when you need to check how how the entire crate should behave with the modules combined.
