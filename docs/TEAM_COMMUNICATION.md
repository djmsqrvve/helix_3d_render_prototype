# 📡 Helix Team Communication Hub

Welcome to the Helix Project! This document defines how **Team Dragon**, **Team V**, and **Team S** collaborate to build the world's best Dota 2 3D renderer prototype.

## 🏰 The Team Structure

| Team | Lead | Focus |
|------|------|-------|
| **Team Dragon** | **User (Head)** | Project Vision, Architecture, Strategic Guidance |
| **Team V** | **Brazilian Lead** | 3D Features, Community Outreach (PT-BR) |
| **Team S** | **French Lead** | Technical 3D Refinement, Community Outreach (FR) |

## 🚀 Communication Routes

### 1. The "Status Sync" (Weekly)
Each team should update their respective `docs/[lang]/STATUS.md` or a new `docs/teams/[team_name].md` weekly.
- **Goal**: What did you build? What's next?
- **Format**: Bullet points + Progress Bar.

### 2. The "Decision Log" (Ongoing)
Before making a major change, post it in `docs/DEV_DECISIONS.md` under **Pending/Proposed**.
- **Rule**: If Team Dragon (Head) doesn't object within 48 hours, the plan is "Soft Approved."

### 3. The "Help Wanted" (Open Questions)
Got stuck on a 3D math problem or a Rust error?
- Post it in the **Questions** section of the relevant module's `README.md` (e.g., `src/render/README.md`).
- Tag your team name so we know who is asking.

### 4. Git Etiquette
- **Team V Branch**: `feature/v-[feature-name]`
- **Team S Branch**: `feature/s-[feature-name]`
- **Team Dragon Branch**: `master` (or `feature/dragon-[...]`)

## 🛠️ Team Tools
- **AI Co-pilot**: Use the `docs/APPRENTICE_AI_GUIDE.md` to help you plan and write code.
- **Guardrails**: Always run `make guardrail` before asking for a review.

---
*Let's build something amazing together!*
