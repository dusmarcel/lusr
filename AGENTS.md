# Repository Guidelines

## Project Structure & Module Organization
- Core Rust code lives in `src/`.
- `src/main.rs` mounts the Leptos app; `src/lib.rs` wires routes, signals, and top-level components.
- UI sections are split into `src/components/` (for example `community.rs`, `income.rs`, `result.rs`).
- Domain calculation logic is in `src/utils.rs`, `src/standardrates.rs`, `src/defaults.rs`, and `src/incomes/`.
- Static web assets are at repo root: `index.html`, `styles.css`.
- Container and deployment files are `Dockerfile`, `compose.yaml`, and `nginx.conf`.
- Build artifacts go to `target/` and should not be edited.

## Build, Test, and Development Commands
- `cargo check` validates Rust code quickly without producing release binaries.
- `cargo test` runs unit/integration tests (currently minimal; still required before PRs).
- `cargo fmt --all` formats code using Rustfmt defaults.
- `cargo clippy --all-targets -- -D warnings` catches lint issues and treats warnings as errors.
- `trunk serve` runs the app locally with live reload.
- `trunk build --release` builds production WASM assets into `dist/`.
- `docker compose up --build` builds and serves the app at `http://localhost:8686`.

## Coding Style & Naming Conventions
- Follow Rust 2024 idioms and keep formatting Rustfmt-clean (4-space indentation, trailing commas where applicable).
- Use `snake_case` for functions/files/modules and `CamelCase` for structs/components.
- Keep components focused; move non-UI calculations into `utils`/domain modules.
- Prefer descriptive German-domain terms already used in the codebase for consistency.

## Testing Guidelines
- Place unit tests next to implementation using `#[cfg(test)] mod tests`.
- Name tests by behavior, e.g. `calculates_absetzbetrag3_with_children_cap`.
- Prioritize calculation edge cases (thresholds like `100`, `520`, `1000`, `1200/1500`) and regression tests for reported bugs.
- Run `cargo test` before opening a PR.

## Commit & Pull Request Guidelines
- Existing history uses short, direct subjects (for example: `deps update`, `fixed typo`, `income bugfix`). Keep commit titles concise and imperative.
- Keep commits scoped to one change.
- PRs should include: purpose, summary of affected modules, test/verification steps run, and screenshots for UI changes.
- Link relevant issues and note any follow-up work explicitly.
