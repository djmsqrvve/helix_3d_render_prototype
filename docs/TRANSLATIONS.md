# 🌐 Helix Translation Project

Welcome to the Helix 3D Renderer translation initiative! This document is your starting point for contributing translations.

---

## 📋 Quick Start for Translators

### 1. Choose Your Language

| Language | Status | Lead | Folder |
|----------|--------|------|--------|
| 🇺🇸 English | ✅ Source | @djmsqrvve | [`docs/en/`](en/) |
| 🇷🇺 Russian | 🚧 In Progress | *Looking for lead* | [`docs/ru/`](ru/) |
| 🇧🇷 Portuguese (BR) | 🚧 In Progress | *Looking for lead* | [`docs/pt-BR/`](pt-BR/) |
| 🇫🇷 French | 🚧 In Progress | *Looking for lead* | [`docs/fr/`](fr/) |
| *Your language?* | 💡 Propose it! | You? | *Create it!* |

### 2. Translation Workflow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   English   │────▶│  Translate  │────▶│    Review   │
│   (Source)  │     │   (You!)    │     │  (Community)│
└─────────────┘     └─────────────┘     └──────┬──────┘
                                                │
                       ┌────────────────────────┘
                       ▼
                ┌─────────────┐
                │   Merge!    │
                │  🎉 Live!   │
                └─────────────┘
```

### 3. Steps to Contribute

1. **Check existing work**: Look at your language folder
2. **Open an Issue**: Comment on [Translation Tracking Issue #XX](../..) to claim a section
3. **Fork & Branch**: `git checkout -b translation/ru/cli-usage`
4. **Translate**: Follow the [Style Guide](#style-guide)
5. **Self-Review**: Use the [Checklist](#review-checklist)
6. **Open PR**: Tag `@djmsqrvve` and any language reviewers

---

## 📁 Folder Structure

```
docs/
├── TRANSLATIONS.md           # This file
├── TRANSLATION_STYLE_GUIDE.md # Detailed guidelines
├── en/                       # 🇺🇸 English (source of truth)
│   ├── README.md
│   ├── DEV_CLI_USAGE.md
│   ├── DEV_TROUBLESHOOTING.md
│   └── DEV_LOGGING.md
├── ru/                       # 🇷🇺 Russian
│   ├── README.md
│   ├── STATUS.md             # Progress tracking
│   ├── CONTRIBUTING.md       # Russian-specific guide
│   ├── DEV_CLI_USAGE.md
│   ├── DEV_TROUBLESHOOTING.md
│   └── DEV_LOGGING.md
└── pt-BR/                    # 🇧🇷 Portuguese (Brazil)
    ├── README.md
    ├── STATUS.md
│   ├── CONTRIBUTING.md       # PT-BR-specific guide
    ├── DEV_CLI_USAGE.md
    ├── DEV_TROUBLESHOOTING.md
    └── DEV_LOGGING.md
```

---

## 🎯 Translation Priority

Translate in this order:

1. **README.md** - First impression, most important
2. **DEV_CLI_USAGE.md** - Most technical value
3. **DEV_TROUBLESHOOTING.md** - Helps users solve problems
4. **DEV_LOGGING.md** - Developer documentation

---

## ✅ Style Guide (Quick Version)

| Rule | Example |
|------|---------|
| **Keep code blocks untranslated** | `cargo run` stays `cargo run` |
| **Keep technical terms** | "GLTF", "shader", "GPU" - add explanation if needed |
| **Translate UI labels** | "Settings" → "Настройки" (Russian) |
| **Use formal vs casual?** | See your language's CONTRIBUTING.md |
| **Update frontmatter** | Add `language: ru` and `translators:` |

**📖 Full guide**: [TRANSLATION_STYLE_GUIDE.md](TRANSLATION_STYLE_GUIDE.md)

---

## 🔍 Review Checklist

Before submitting your PR, verify:

- [ ] All code blocks are unchanged
- [ ] Links work (use relative paths)
- [ ] Images display correctly
- [ ] Technical terms have consistent translations
- [ ] Frontmatter is filled out
- [ ] You've run `make check-docs` (if available)

---

## 👑 Language Lead Responsibilities

Want to be a **Language Lead**? You'll:

1. **Coordinate** translation efforts for your language
2. **Review** PRs from other translators
3. **Maintain** STATUS.md with progress
4. **Resolve** translation disputes (style, terminology)
5. **Sync** with main repo when English docs change

**Perks**: Listed in README, commit access to translation branch, recognition in release notes.

**To apply**: Open an issue titled "[Translation Lead] Apply for [Language]"

---

## 🔄 Keeping Translations in Sync

When English docs change, we:

1. Tag the change with `i18n-impact` label
2. Notify Language Leads
3. Update `STATUS.md` with "Needs Update" marker
4. Translator updates and PRs

---

## 🛠️ Tools (Optional Future)

We may integrate:
- **GitLocalize** - Crowdsourced platform
- **Weblate** - Open source alternative
- **Custom scripts** - Sync checking, validation

For now: **Git + GitHub PRs** work great!

---

## 💬 Questions?

- **General**: Comment on this issue or discussion
- **Language-specific**: See your language's CONTRIBUTING.md
- **Urgent**: Ping @djmsqrvve

---

## 🏰 The Team Structure

| Team | Lead | Focus |
|------|------|-------|
| **Team Dragon** | **User (Head)** | Project Vision, Architecture, Strategic Guidance |
| **Team V** | **Brazilian Lead** | 3D Features, Community Outreach (PT-BR) |
| **Team S** | **French Lead** | Technical 3D Refinement, Community Outreach (FR) |

See the [Communication Hub](TEAM_COMMUNICATION.md) for how we collaborate.

---

## 🏆 Hall of Fame

### Russian (ru)
- Lead: *Open position*
- Contributors: *Be the first!*

### Portuguese - Brazil (pt-BR)
- Lead: **Team V** (Brazilian Student)
- Contributors: Team V Members

### French (fr)
- Lead: **Team S** (French Student)
- Contributors: Team S Members

---

**Thank you for making Helix accessible worldwide! 🌍**
