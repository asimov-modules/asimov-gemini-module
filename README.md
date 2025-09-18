# ASIMOV Gemini Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-gemini-module)](https://crates.io/crates/asimov-gemini-module)
[![Documentation](https://docs.rs/asimov-gemini-module/badge.svg)](https://docs.rs/asimov-gemini-module)

[ASIMOV] Gemini module.

## ✨ Features

- To be determined!

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code

## ⬇️ Installation

### Installation with [ASIMOV CLI]

```bash
asimov module install gemini -v
```

### Installation from Source Code

```bash
cargo install asimov-gemini-module
```

## 👉 Examples

```bash
asimov-gemini-prompter
```

## ⚙ Configuration

Provide an API key either by module configuration

```bash
asimov module config gemini
```

Or through environment variables

```bash
export GEMINI_API_KEY="..."
```

### Optional configuration

| Name       | Environment Variable  | Default                                     |
| ---------- | --------------------- | ------------------------------------------- |
| `endpoint` | `GEMINI_API_ENDPOINT` | `https://generativelanguage.googleapis.com` |
| `model`    | `GEMINI_MODEL`        | `gemini-2.5-flash`                          |

## 📚 Reference

### Prompt

```bash
echo "Why is the sky blue?" | asimov-gemini-prompter
```

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-gemini-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-gemini-module&text=asimov-gemini-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-gemini-module&title=asimov-gemini-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-gemini-module&t=asimov-gemini-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-gemini-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-gemini-module)

[ASIMOV]: https://asimov.sh
[ASIMOV CLI]: https://cli.asimov.sh
[JSON-LD]: https://json-ld.org
[KNOW]: https://know.dev
[RDF]: https://www.w3.org/TR/rdf12-primer/
[Rust]: https://rust-lang.org
