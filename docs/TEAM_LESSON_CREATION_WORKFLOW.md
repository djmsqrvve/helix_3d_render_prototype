# 🎓 Lesson Creation Workflow

This guide is for **Team Dragon** to create consistent, high-quality lessons for our apprentices (Team V and Team S).

## 🛠️ The 4-Step Process

### 1. Design the Objective
What should the apprentice learn? 
- **Team V**: Focus on simple edits, terminology, and "seeing" changes.
- **Team S**: Focus on technical structure, performance, and advanced features.

### 2. Draft the Document
Create a new file in `docs/` using the correct prefix:
- `TEAMV_LESSON_N_NAME.md`
- `TEAMS_LESSON_N_NAME.md`

**Structure**:
- **🎯 Goal**: One clear sentence.
- **🛠️ Steps**: Numbered list with exact file names and commands.
- **🎉 Success**: How do they know they did it right?

### 3. The "snowcamo" Dry Run
**Crucial**: Never ship a lesson you haven't done yourself as an apprentice.
1. Switch identity: `git config --local user.name "snowcamo"`.
2. Follow your own instructions exactly.
3. If anything is confusing or breaks, fix the doc.

### 4. Publish & Notify
1. Merge the new lesson to `main`.
2. Update `docs/TEAM_MATRIX.md` to show the new lesson is available.
3. Inform the team leads during the stream or sync.

---
## 💡 Pro-Tips for AI
Ask the AI to help you draft:
> "AI, draft a Lesson 2 for Team V. The goal is to change the clear color of the 3D scene in `src/main.rs`. Keep it very simple and use emojis."
