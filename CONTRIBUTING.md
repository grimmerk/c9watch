# Contributing to c9watch

Thanks for your interest in contributing to c9watch! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Contribution Workflow](#contribution-workflow)
- [Coding Standards](#coding-standards)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help create a welcoming environment for all contributors

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue with:
- A clear, descriptive title
- Steps to reproduce the issue
- Expected vs actual behavior
- Your environment (macOS version, c9watch version)
- Screenshots if applicable

### Suggesting Features

Feature requests are welcome! Please:
- Check if the feature has already been requested
- Explain the use case and why it would be valuable
- Provide examples of how it would work

### Contributing Code

We welcome code contributions! Areas where we especially need help:
- **Platform support**: Windows and Linux ports
- **Performance**: Optimization of session detection and parsing
- **UI improvements**: Better visualizations and user experience
- **Bug fixes**: Check our [issues](https://github.com/minchenlee/c9watch/issues)

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) v18 or higher
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your platform

### Initial Setup

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/c9watch.git
cd c9watch

# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Project Structure

```
c9watch/
├── src/                    # SvelteKit frontend (TypeScript, Svelte)
│   ├── routes/             # Pages
│   ├── lib/
│   │   ├── components/     # Reusable Svelte components
│   │   ├── stores/         # Reactive state
│   │   └── api.ts          # Tauri command wrappers
├── src-tauri/              # Rust backend
│   └── src/
│       ├── lib.rs          # Main entry point
│       ├── polling.rs      # Session detection loop
│       └── session/        # Session-related logic
```

## Contribution Workflow

### 1. Fork and Clone

Fork the repository to your GitHub account, then clone your fork.

### 2. Create a Branch

Always create a feature branch for your work:

```bash
# For new features
git checkout -b feature/your-feature-name

# For bug fixes
git checkout -b fix/bug-description

# For documentation
git checkout -b docs/what-you-are-documenting
```

### 3. Make Changes

- Write clear, readable code
- Follow existing code style
- Add tests if applicable
- Update documentation if needed

### 4. Test Your Changes

```bash
# Run the app in dev mode
npm run tauri dev

# Build to test production build
npm run tauri build

# Test the built app
open src-tauri/target/release/bundle/macos/c9watch.app
```

### 5. Commit Your Changes

See [Commit Messages](#commit-messages) section below.

### 6. Push and Create PR

```bash
# Push to your fork
git push -u origin feature/your-feature-name
```

Then open a Pull Request on GitHub from your fork to the main repository.

## Coding Standards

### Rust Code

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Keep functions small and focused
- Add comments for complex logic

```bash
# Format code
cd src-tauri && cargo fmt

# Check for issues
cd src-tauri && cargo clippy
```

### TypeScript/Svelte Code

- Use TypeScript for type safety
- Follow existing component structure
- Keep components small and reusable
- Use Svelte 5 runes (`$state`, `$derived`, `$effect`)

### Style

- Use tabs for indentation (project convention)
- Keep lines under 100 characters when reasonable
- Add blank lines to separate logical sections

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>: <description>

[optional body]

[optional footer]
```

### Types

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `refactor:` - Code refactoring (no behavior change)
- `perf:` - Performance improvements
- `test:` - Adding or updating tests
- `chore:` - Build process, dependencies, tooling

### Examples

```
feat: add Windows support for session detection

fix: prevent crash when session file is corrupted

docs: update installation instructions for Linux

refactor: simplify status determination logic
```

## Pull Request Process

### Before Submitting

- [ ] Code builds successfully (`npm run tauri build`)
- [ ] App runs without errors (`npm run tauri dev`)
- [ ] Code follows project style (run formatters)
- [ ] Commit messages follow convention
- [ ] Branch is up to date with `main`

```bash
# Update your branch with latest main
git fetch upstream
git rebase upstream/main
```

### PR Description

Include:
- **What** - What does this PR do?
- **Why** - Why is this change needed?
- **How** - How did you implement it?
- **Testing** - How did you test it?
- **Screenshots** - If UI changes, include before/after screenshots

### Review Process

1. Maintainer will review your PR
2. Address any requested changes
3. Once approved, maintainer will merge your PR
4. Your contribution will be included in the next release!

### After Merge

Your branch will be automatically deleted. You can safely delete your local branch:

```bash
git checkout main
git branch -d feature/your-feature-name
```

## Getting Help

- Open an issue for bugs or feature requests
- Ask questions in PR comments
- Check existing issues and PRs for similar discussions

## Platform-Specific Contributions

### macOS (currently supported)
- Test on different macOS versions if possible
- Ensure compatibility with both Intel and Apple Silicon

### Linux (in progress - see PR #2)
- Help test and refine Linux support
- Document distribution-specific issues

### Windows (in progress - see PR #1)
- Help test and refine Windows support
- Document Windows-specific requirements

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
