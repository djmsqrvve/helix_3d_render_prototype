# 🧪 Lead's Dry Run Guide

This guide is for **Team Dragon** to test lessons using a secondary account (like `snowcamo`) before they are released to the apprentices.

## 👤 Step 1: Switch Identity
To act like a student, change your local git identity for this folder only:

```bash
# Set your secondary identity
git config --local user.name "snowcamo"
git config --local user.email "djmsqrvve@gmail.com" # Or your secondary email

# Verify it changed
git config user.name
```

## 🛠️ Step 2: The "Lesson 1" Test
Perform the steps exactly as written in `docs/TEAMV_LESSON_1_SIGNING_OFF.md`:
1. Edit `HALL_OF_HEROES.md`.
2. Commit with the message `feat: snowcamo signing in!`.
3. Push to a test branch: `git push origin main`.

## 📊 Step 3: Log Your Test
Mark your dry runs in the **Testing Matrix** below to track which lessons are "Production Ready."

### Lesson Readiness Matrix
| Lesson | Dry Run (snowcamo) | Status |
|--------|--------------------|--------|
| Lesson 1: Signature | [x] | ✅ Ready |
| Lesson 2: Constants | [ ] | 🏗️ Testing |

## 🔄 Step 4: Reset Identity
When finished, you can return to your main identity:
```bash
git config --local --unset user.name
git config --local --unset user.email
```
