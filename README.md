# 🦀 url-to-md

> A fast, offline-friendly CLI tool to convert blog posts and documentation pages into Markdown files for use in codebases, AI assistants (like Cursor or Windsurf), or personal notes.

---

## ✨ Features

- 🌐 Takes one or more URLs and downloads the page content
- 🧠 Extracts the `<article>` content (or falls back to the full HTML body)
- 🔄 Converts HTML to clean, readable Markdown
- 📂 Saves to `./docs/internal/{slugified-title}.md` with frontmatter
- ⚡ Built in Rust — fast, portable, no runtime dependencies

---

## 💡 Why?

Modern developer tools are increasingly powered by AI. Tools like [Cursor](https://cursor.sh) and [Windsurf](https://windsurf.ai) can read context from files in your repo. By converting web-based docs or blog posts into Markdown files and saving them in your project, this tool helps you:

- 📚 Teach your AI assistant how and why your team uses certain patterns
- 🧠 Provide architectural guidance or reasoning directly in the repo
- 🪄 Reduce copy-pasting and memory overload — keep docs close to code

---

## 🤖 Using with AI-Powered Editors

### 🌀 Cursor

Cursor reads your entire codebase context. Use `url-to-md` to store high-value docs in your repo.

**Steps:**
1. Run `url_to_md <url>` to convert blog posts or docs into Markdown.
2. Save them to `docs/internal/`.
3. Refer to them in code comments:
   ```ts
   // See: docs/internal/state-handling-patterns.md
   ```

Cursor will automatically pick these up during code suggestions and inline chat.

> 🧠 Pro tip: You can also copy-paste them into `.cursor/context.ts` for stronger emphasis.

---

### 🌊 Windsurf AI

Windsurf reads repo-wide context and uses long-context LLMs to power coding sessions.

Use `url-to-md` to:
- Seed your repo with foundational knowledge
- Link implementation decisions to external sources
- Provide AI-consumable rationale behind architecture choices

No extra config needed — just commit the generated files.

---

## 🚀 Getting Started

### 🔧 Build from source

```bash
git clone https://github.com/yourname/url-to-md.git
cd url-to-md
cargo install --path .
```

Now you can use the CLI globally:

```bash
url_to_md https://example.com/blog-post
```

---

## 🛠 Usage

```bash
url_to_md <url1> [url2] [url3] ...
```

Example:

```bash
url_to_md \
  https://kentcdodds.com/blog/how-to-write-a-react-component \
  https://react.dev/learn/managing-state
```

Markdown files will be saved to:

```
./docs/internal/
```

Each file contains metadata:

```markdown
---
title: "Managing State in React"
source: "https://react.dev/learn/managing-state"
---

# Managing State in React

...markdown content...
```

---

## 📁 Project Structure

| File/Folder         | Description                          |
|---------------------|--------------------------------------|
| `src/main.rs`       | Main CLI implementation              |
| `docs/internal/`    | Output folder for converted Markdown |
| `Cargo.toml`        | Rust project config + dependencies   |

---

## 📦 Dependencies

This tool is powered by:

- [`reqwest`](https://crates.io/crates/reqwest) — HTTP requests
- [`select`](https://crates.io/crates/select) — HTML element selection
- [`html2md`](https://crates.io/crates/html2md) — HTML → Markdown
- [`slug`](https://crates.io/crates/slug) — Converts titles to filename-safe slugs
- [`url`](https://crates.io/crates/url) — URL parsing & validation

---

## 📦 Installation as a Global CLI

Once built, you can install it system-wide:

```bash
cargo install --path .
```

Then use it from any terminal:

```bash
url_to_md <url>
```

---

## 🧠 Ideas for Future Enhancements

- `--output-dir` flag for custom output location
- Accept input from a `.txt` file or stdin
- Add `README.md` index generation for all saved docs
- Download and inline images (optionally)
- Save original HTML for comparison/debugging

---

## 🤝 Contributing

All contributions welcome!

1. Fork the repo
2. Create a feature branch
3. Submit a pull request

Please open an issue if you're planning a large feature to avoid duplication.

---

## 🧪 Example: AI-Friendly Internal Docs

```bash
url_to_md https://example.com/why-we-use-functional-core
```

Saves as:

```markdown
docs/internal/why-we-use-functional-core.md
```

And you can now reference this in your code like:

```ts
// This pattern is explained in docs/internal/why-we-use-functional-core.md
```

---

## 📄 License

MIT License — free for personal and commercial use.

---

## ✍️ Author

Made by [Matt Sell](https://atomicbytes.com)
Follow me on [GitHub](https://github.com/msell)
