# osmia
osmia is a clean, expressive templating language rust library designed for
building dynamic content in any programming language. Whether you're
generating HTML, SQL, JSON, Java, or configuration files - osmia helps you
write templates that are readable, maintainable, and decoupled from your
application logic.

## Documentation / Docs:
[Official Documentation](https://jkutkut.github.io/osmia/osmia/index.html)
[Repository](https://github.com/jkutkut/osmia)

## What is osmia?
osmia is a template interpreter — a small, fast engine that reads template files
containing dynamic expressions, control flow, and text content, then renders
them into plain text output based on the given data.

What sets osmia apart is its language independence. It isn’t tied to any
particular language ecosystem. You can embed osmia templates in any
project, whether you’re working in Java, Python, C#, Rust, Go, or any other
language. This makes it perfect for projects that need flexible template
generation without being locked into a specific tech stack.

| Project | Description |
| --- | --- |
| [osmia-vscode](https://github.com/jkutkut/osmia-vscode) | Extension for VSCode |
| [osmia-cli](https://github.com/jkutkut/osmia-cli) | CLI tool |
| [docker4osmia](https://github.com/jkutkut/docker4osmia) | Docker image with osmia-cli |

## Use Cases
- HTML templating in web projects.
- Formatting JSON from an API response.
- Code generation for boilerplate in any language.
- Configuration generation for CI/CD, Kubernetes, etc.
- Documentation templates with embedded dynamic data.
- Scripting and scaffolding tools.
If you need to render structured files from templates — osmia fits right in.

## Special thanks:
This code was inspired from [Crafting Interpreters](https://craftinginterpreters.com/), by Robert Nystrom.
