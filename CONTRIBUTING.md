# Contributing to Helix

Welcome to the Helix 3D Renderer project! To maintain the quality and focus of our prototype, please follow these guardrails.

## 🛡️ The Golden Rule
**The 3D hardware-accelerated render is the #1 priority.** Master branch must ALWAYS show the 3D model. If a change breaks the render, it cannot be merged.

## 🚦 Pre-Commit Guardrail Checklist
Before you commit any code, you **must** run these checks:

1.  **Build Check**: `cargo build` (No warnings)
2.  **Formatting**: `make format-fix`
3.  **Linter**: `make lint` (Must be clean)
4.  **Tests**: `make test` (Must pass)
5.  **Visual Verify**: `make dev` (Confirm the 3D model still renders correctly)
6.  **Strict Check**: `make guardrail-strict` (Highly recommended before PR)

## 📁 Repository Structure
- `src/`: Core logic. Each module has its own `README.md` with specific guardrails.
- `docs/`: Technical and platform documentation.
- `assets/`: 3D models and textures.
- `tests/`: Integration and regression tests.

## 🛠️ Feature Workflow
- NEVER work on `master` directly.
- Create a feature branch: `git checkout -b feature/your-feature`.
- Keep changes minimal and focused. Do not refactor unrelated code.
- Update relevant documentation and add tests for new features.

## 💬 Decisions & Questions
- Consult `docs/DEV_DECISIONS.md` before making major architectural changes.
- Add open questions to the `README.md` of the relevant module or `docs/DEV_DECISIONS.md`.

## 🌐 Translations
- Follow the [Translation Style Guide](docs/TRANSLATION_STYLE_GUIDE.md).
- Update the `STATUS.md` in your language's directory.

---
*Thank you for contributing! Let's build a rock-solid 3D renderer.*
