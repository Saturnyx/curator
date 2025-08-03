# Contributing Guidlines

Firstly, thank you for considering contributing to this project. It really means
a lot to me. The following text are guidelines rather than rules try to adhere
to as much of them but you don't have to strictly follow them.

## Getting Started

- Fork the repository and create your branch from `main`.
- Make your changes with [clear commit messages](#commits)
- Ensure your code follows the [project's style](#project-style) and passes all
  tests.
- Submit a pull request with a clear description of your changes.

### Commits

For commits please use semantic commit messages as they are more informative and
help in understanding the changes made. Here is a template you can use:

```bash
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

More information on semantic commits can be found on this
[page](https://www.conventionalcommits.org/en/v1.0.0/).

### Project Style

Before commiting or sending a pull request please ensure that your code follows
the project's style. You can use `cargo fmt` to format your code and
`cargo clippy` to check for any warnings or suggestions. This will help maintain
consistency across the codebase.

## Code of Conduct

Please read our [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before contributing.

Thank you!

> **Summary**:
>
> - Use semantic commit messages
> - Follow project style with `cargo fmt` and `cargo clippy`
> - Ensure your changes are well-tested
