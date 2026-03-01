# 🇫🇷 Guide pour les traducteurs en français

Directives spécifiques pour traduire Helix en français.

---

## 🎯 Style et ton

- **Formalité** : Semi-formel (utilisez "vous", pas "tu")
- **Ton** : Professionnel, mais accessible
- **Public cible** : Développeurs, éventuellement non familiers avec l'industrie du jeu vidéo

---

## 📚 Glossaire (initial)

| Anglais | Français | Remarques |
|---------|----------|-----------|
| Renderer | Moteur de rendu | "3D renderer" → "moteur de rendu 3D" |
| 3D rendering | Rendu 3D | Ou "visualisation tridimensionnelle" |
| Hardware-accelerated | Accéléré par matériel | |
| Skeletal animation | Animation squelettique | Ou "animation par squelette" |
| Wireframe | Mode filaire | Dans le contexte du débogage |
| Skeleton | Squelette / Structure osseuse | |
| Bone | Os | |
| Accessory | Accessoire | Armure, arme, etc. |
| Accessory attachment | Attachement des accessoires | |
| Debug visualization | Visualisation de débogage | |
| Shader | Shader | Terme technique, non traduit |
| GPU | GPU | On peut utiliser l'acronyme |
| Toggle | Basculer / Activer/désactiver | Selon le contexte |
| CLI | CLI | Ou "ligne de commande" |
| Screenshot | Capture d'écran | |

---

## 📝 Particularités de la traduction en français

### 1. Tutoiement ou vouvoiement

Utilisez "vous" (formel mais professionnel en français technique) :

```markdown
<!-- ✅ Correct -->
Appuyez sur la touche `F1` pour activer le mode filaire.
Ajoutez le flag `--animation` pour sélectionner une animation.

<!-- ❌ Incorrect -->
Appuie sur la touche `F1` pour activer le mode filaire.  (trop familier)
```

### 2. Termes techniques

Gardez l'original entre parenthèses à la première mention :

```markdown
Le moteur de rendu utilise l'animation squelettique (skeletal animation) pour...
```

### 3. Commandes et code

NE traduisez PAS les commandes :

```markdown
<!-- ✅ Correct -->
Exécutez la commande :
```bash
cargo run -- --animation 2
```

<!-- ❌ Incorrect -->
Exécutez la commande :
```bash
cargo exécuter -- --animation 2
```
```

### 4. Espaces insécables

En français typographique, utilisez des espaces insécables avant :
- Les deux-points `:`
- Les points-virgules `;`
- Les points d'interrogation `?`
- Les points d'exclamation `!`

```markdown
<!-- ✅ Correct -->
Appuyez sur la touche `F1` pour activer le mode filaire.

<!-- ❌ Incorrect (en typographie française stricte) -->
Appuyez sur la touche `F1` pour activer le mode filaire.
```

---

## 🔍 Vérification de qualité

Avant d'envoyer le PR :

- [ ] Le texte sonne naturel (ne ressemble pas à une traduction automatique)
- [ ] Toutes les commandes sont en anglais
- [ ] Les termes techniques sont cohérents avec le glossaire
- [ ] Les liens fonctionnent
- [ ] L'accord des participes passés est vérifié

---

## 💬 Discussions

Des questions sur la traduction ?
- Ouvrez une Discussion avec le tag `translation-fr`
- Ou demandez dans la [discussion principale sur les traductions](../../..)

---

**Merci de contribuer ! 🎉**
