# Helix 3D Renderer - AI Agent & Technical Guidelines

This document provides technical rules for AI agents (like Gemini CLI, Cursor, or GPT-4) and automated tools interacting with the Helix codebase.

## 🤖 AI Interaction Rules (Technical)
1. **Always Research First**: Before suggesting code, analyze the existing module `README.md` and `docs/DEV_DECISIONS.md`.
2. **The "Plan Mode" Protocol**: For any task larger than a single function fix, the AI must create a `plan.md` file in the project's temporary directory (or `docs/plans/`) and wait for human approval.
3. **No "Magic Numbers"**: AI must use constants from `src/constants.rs` for animation indices, model scales, and UI layout values.
4. **Performance is a Feature**: If adding a new system, the AI must ensure it doesn't re-calculate the same data every frame (use Bevy's change detection).

## 🛠️ Automated Testing & Guardrails
All AI-generated code **must** pass the project's built-in guardrails:
- `make dev`: Confirms the 3D render doesn't crash on startup.
- `make guardrail`: Checks formatting, linter, and basic compilation.
- `make guardrail-strict`: Exhaustive checks including clippy warnings.

## 📁 Project Knowledge
- **Target OS**: Ubuntu 22.04+ (Primary).
- **Engine**: Bevy 0.18.
- **Cross-Compilation**: Use `make build-windows` to test Windows compatibility.

## 📝 Commit Messages for AI
AI agents should follow this format for all commits:
`<type>: <description>`
- `feat`: A new feature (e.g., `feat: add accessory toggle panel`)
- `fix`: A bug fix (e.g., `fix: resolve bone mismatch in cape attachment`)
- `docs`: Documentation updates (e.g., `docs: add AI contributor guide`)
- `chore`: Cleanup or build system changes (e.g., `chore: optimize Windows linking with lld`)

---
*For human-to-AI collaboration tips, see `docs/APPRENTICE_AI_GUIDE.md`.*
