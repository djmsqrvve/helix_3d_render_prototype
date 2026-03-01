# 🇧🇷 Guia para Tradutores em Português Brasileiro

Diretrizes específicas para traduzir o Helix para português brasileiro.

---

## 🎯 Estilo e Tom

- **Formalidade**: Semi-formal (use "você", não "tu")
- **Tom**: Profissional, mas acessível
- **Público-alvo**: Desenvolvedores, possivelmente não familiarizados com a indústria de jogos

---

## 📚 Glossário (inicial)

| Inglês | Português (BR) | Observações |
|--------|----------------|-------------|
| Renderer | Renderizador | "3D renderer" → "renderizador 3D" |
| 3D rendering | Renderização 3D | Ou "visualização tridimensional" |
| Hardware-accelerated | Acelerado por hardware | |
| Skeletal animation | Animação esquelética | Ou "animação por esqueleto" |
| Wireframe | Modo arame | No contexto de depuração |
| Skeleton | Esqueleto / Estrutura óssea | |
| Bone | Osso | |
| Accessory | Acessório | Armadura, arma, etc. |
| Accessory attachment | Anexação de acessórios | |
| Debug visualization | Visualização de depuração | |
| Shader | Shader | Termo técnico, não traduzido |
| GPU | GPU | Pode usar a sigla |
| Toggle | Alternar / Ativar/desativar | Depende do contexto |
| CLI | CLI | Ou "linha de comando" |
| Screenshot | Captura de tela | Ou "screenshot" (comum) |

---

## 📝 Particularidades da tradução para PT-BR

### 1. Tratamento

Use "você" (informal, mas profissional no Brasil):

```markdown
<!-- ✅ Correto -->
Pressione a tecla `F1` para ativar o modo arame.
Adicione a flag `--animation` para selecionar uma animação.

<!-- ❌ Incorreto -->
Pressione a tecla `F1` para ativar o modo arame, por favor.  (muito formal)
Pressiona a tecla `F1`...  (tratamento inadequado)
```

### 2. Termos técnicos

Mantenha o original entre parênteses na primeira menção:

```markdown
O renderizador usa animação esquelética (skeletal animation) para...
```

### 3. Comandos e código

NÃO traduza comandos:

```markdown
<!-- ✅ Correto -->
Execute o comando:
```bash
cargo run -- --animation 2
```

<!-- ❌ Incorreto -->
Execute o comando:
```bash
cargo executar -- --animação 2
```
```

---

## 🔍 Verificação de qualidade

Antes de enviar o PR:

- [ ] Texto soa natural (não parece tradução automática)
- [ ] Todos os comandos estão em inglês
- [ ] Termos técnicos consistentes com o glossário
- [ ] Links funcionam
- [ ] Verificada a concordância verbal

---

## 💬 Discussões

Dúvidas sobre tradução?
- Abra uma Discussion com a tag `translation-pt-BR`
- Ou pergunte na [discussão principal de traduções](../../..)

---

**Obrigado por contribuir! 🎉**
