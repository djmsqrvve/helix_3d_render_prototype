# 🤖 AI Contributor Guide

Welcome! Helix is designed to be built with AI assistance. This guide will help you use AI (Gemini, ChatGPT, Claude, etc.) to understand the code, create plans, and contribute safely.

## 💡 The "Plan First" Rule
**NEVER** ask an AI to "just fix it" or "write the code" immediately. Always follow this workflow:

1.  **Ask for Analysis**: "Explain how the accessory attachment works in `src/model/accessories.rs`."
2.  **Ask for a Plan**: "I want to add a new 'glow' effect to the weapon. Give me a step-by-step implementation plan."
3.  **Review the Plan**: Make sure it doesn't break our **Golden Rule** (3D render must always work).
4.  **Execute & Test**: Ask for the code for *one step at a time* and run `make guardrail` after each.

## 📝 How to Prompt for Helix
To get the best results, always give the AI context. Copy and paste this "Context Header" before your request:

> "I am working on Helix, a 3D renderer prototype built with Rust and Bevy 0.18. The project uses an Ubuntu-first focus and follows strict guardrails in each module's README. I have a `Makefile` with a `make guardrail` command for testing."

## 🛠️ Using AI for Git
AI is great at helping you manage your contributions:

-   **Commit Messages**: "I just updated the camera zoom logic in `src/camera/controller.rs`. Write a concise, professional Git commit message following the project style."
-   **Code Reviews**: Paste your changes and ask: "Does this code follow Rust best practices and avoid O(N²) performance issues?"

## 💻 Recommended AI Setup (VS Code)
For the best team experience, use **VS Code** with these AI integrations:
- **GitHub Copilot**: Excellent for real-time Rust completions.
- **Cursor**: A VS Code fork that understands the entire codebase (highly recommended for complex 3D tasks).
- **Error Lens**: (Recommended Extension) Highlights errors directly in your line of code so you can quickly paste them into an AI for fixing.

## 🛡️ AI Guardrails (What to Watch For)
AI often makes these mistakes in Bevy. Be careful:
-   **Outdated Versions**: AI might give you Bevy 0.12 code. We use **0.18**.
-   **Missing Features**: If the AI uses a crate we don't have, check `Cargo.toml` first.
-   **Over-Refactoring**: If the AI suggests rewriting half the file, tell it: "Keep changes minimal and focused only on the task."

## 🚀 Step-by-Step Contribution Example
1.  **Human**: "AI, I want to add a key (F7) to hide the UI."
2.  **AI**: (Provides a plan: 1. Update `src/input/keyboard.rs`, 2. Update `UiWindowState`...)
3.  **Human**: "Write the code for step 1 only."
4.  **AI**: (Provides code)
5.  **Human**: (Applies code, runs `make dev`, confirms F7 works).
6.  **Human**: "Write the code for step 2."

---
*Remember: The AI is your co-pilot, but you are the Captain. Always verify!*
