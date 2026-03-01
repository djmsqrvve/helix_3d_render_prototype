# 📖 Translation Style Guide

Complete guidelines for translating Helix documentation.

---

## 🎯 Core Principles

1. **Accuracy over elegance** - Correct meaning beats beautiful prose
2. **Consistency is king** - Use the same translation for the same term
3. **Know your audience** - Developers who may not know English well
4. **Keep it actionable** - Users should be able to follow steps

---

## 📋 Document Structure

Every translated document must include this frontmatter:

```markdown
---
language: ru                    # ISO 639-1 code
translators:                    # List all contributors
  - name: "Ivan Petrov"
    github: "@ivanpetrov"
  - name: "Maria Silva"
    github: "@mariasilva"
last_updated: "2026-02-23"      # Date of last change
synced_to: "dcd9d06"            # Git commit of English version
status: "complete"              # complete | in-progress | needs-update
reviewers:                      # Who checked this
  - "@reviewer1"
---
```

---

## 📝 Translation Rules

### 1. Code Blocks: NEVER Translate

```markdown
<!-- ✅ CORRECT -->
```bash
cargo run -- --animation 2
```

<!-- ❌ WRONG -->
```bash
карго ран -- --анимация 2   # Don't do this!
```
```

### 2. Technical Terms: Keep or Translate?

| Term | Treatment | Example (Russian) |
|------|-----------|-------------------|
| **Command names** | Keep English | `cargo`, `git`, `make` |
| **File formats** | Keep English | `GLTF`, `PNG`, `README` |
| **API names** | Keep English | `bevy_gltf`, `wgpu` |
| **Computer terms** | Translate with note | GPU → ГП (GPU) |
| **Common UI terms** | Translate | Settings → Настройки |

**Rule of thumb**: If a developer would Google it in English, keep it English.

### 3. Keyboard Shortcuts

Keep the key, translate the description:

```markdown
<!-- ✅ CORRECT -->
Press `F1` to toggle wireframe (Нажмите `F1`, чтобы включить каркасный режим)

<!-- ❌ WRONG -->
Press `Ф1` to toggle wireframe   # Don't translate F1
```

### 4. Links

Use **relative links** for internal docs:

```markdown
<!-- ✅ CORRECT -->
See [CLI Usage](DEV_CLI_USAGE.md)
See [English version](../en/DEV_TROUBLESHOOTING.md)

<!-- ❌ WRONG -->
See [CLI Usage](https://github.com/.../DEV_CLI_USAGE.md)  # Don't use full URLs
```

### 5. Images

Reference the same images (don't duplicate):

```markdown
![Screenshot](../assets/screenshot.png)  # Go up to docs/, then to assets/
```

If you need language-specific screenshots, name them:
```
screenshot-cli-ru.png
screenshot-cli-pt-BR.png
```

---

## 🗣️ Tone and Formality

### Choose Your Tone Per Language

| Language | Recommended Tone | Example |
|----------|------------------|---------|
| Russian | Formal (Вы) | "Нажмите кнопку, чтобы запустить" |
| Portuguese (BR) | Semi-formal | "Clique no botão para iniciar" |
| Japanese | Formal (敬語) | 「ボタンをクリックして起動してください」 |
| Spanish | Semi-formal | "Haz clic en el botón para iniciar" |

**Document your choice** in `docs/[lang]/CONTRIBUTING.md`

---

## 📚 Glossary Template

Create a `GLOSSARY.md` in your language folder:

```markdown
# Русский Глоссарий (Russian Glossary)

| English | Russian | Context |
|---------|---------|---------|
| Renderer | Рендерер | "3D renderer" → "3D-рендерер" |
| Wireframe | Каркасный режим | Debug visualization |
| Skeleton | Скелет | Bone hierarchy |
| Accessory | Аксессуар | Model parts (armor, weapon) |
| Animation | Анимация | - |
| Toggle | Переключить | "Toggle visibility" |
```

---

## 🔍 Common Pitfalls

### 1. False Friends

| Word | Don't Translate As | Correct Translation |
|------|--------------------|--------------------|
| "Actually" (en) | "Actualmente" (pt: currently) | "Na verdade" |
| "Event" (en) | "Evento" (pt: occasion) | "Evento" (programming) ✓ |

### 2. Directional Terms

```markdown
"Above" / "Below" in code examples:
- Refers to position in file, not literally up/down
- Translate contextually: "выше" / "ниже" (Russian)
```

### 3. Placeholder Text

Keep placeholders untranslated:

```markdown
Replace `PATH` with your path
↓
Замените `PATH` на ваш путь   # Keep PATH as is
```

---

## ✅ Quality Checklist

Before marking a document "complete":

- [ ] All code blocks unchanged
- [ ] All links work (test by clicking)
- [ ] Frontmatter filled correctly
- [ ] Technical terms consistent with glossary
- [ ] No machine-translation artifacts
- [ ] Native speaker reviewed
- [ ] Commands tested in your language environment

---

## 🚨 When to Ask for Help

Ping @djmsqrvve or Language Lead when:

- English phrase has multiple meanings (ambiguous)
- Technical concept doesn't exist in your language
- You're unsure about formality level
- You want to change a previously translated term

---

## 📖 Resources

- [Microsoft Style Guide](https://docs.microsoft.com/en-us/style-guide/welcome/) - General tech writing
- [Google Developer Documentation](https://developers.google.com/style) - Another reference
- [ISO Language Codes](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) - For naming folders

---

**Remember**: Good translation helps users. Perfect translation takes time. Ship improvements iteratively! 🚀
