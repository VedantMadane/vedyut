# Contributing to Vedyut

Thank you for your interest in contributing to Vedyut! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites

- **Rust** (1.70+): [Install Rust](https://rustup.rs/)
- **Python** (3.10+): [Install Python](https://www.python.org/downloads/)
- **uv**: [Install uv](https://docs.astral.sh/uv/getting-started/installation/)
- **Git**: For version control

### Setting Up Development Environment

1. **Fork and clone the repository:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/vedyut.git
   cd vedyut
   ```

2. **Set up Python environment:**
   ```bash
   uv venv
   source .venv/bin/activate  # On Windows: .venv\Scripts\activate
   uv sync --all-extras
   ```

3. **Build Rust workspace:**
   ```bash
   cd rust
   cargo build
   cargo test --all
   ```

4. **Run API server:**
   ```bash
   cd ..
   uvicorn python.vedyut.api.main:app --reload
   ```

5. **Run tests:**
   ```bash
   # Python tests
   pytest tests/ -v

   # Rust tests
   cd rust
   cargo test --all
   ```

## Development Workflow

### Branch Naming

- `feature/` - New features (e.g., `feature/add-vedic-support`)
- `fix/` - Bug fixes (e.g., `fix/segmentation-error`)
- `docs/` - Documentation updates (e.g., `docs/api-examples`)
- `perf/` - Performance improvements (e.g., `perf/optimize-sandhi`)

### Making Changes

1. **Create a new branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** and write tests

3. **Run checks:**
   ```bash
   # Format Python code
   ruff format .
   
   # Lint Python code
   ruff check .
   
   # Format Rust code
   cd rust
   cargo fmt --all
   
   # Lint Rust code
   cargo clippy --all-targets --all-features
   ```

4. **Run tests:**
   ```bash
   # Python tests
   pytest tests/ -v --cov
   
   # Rust tests
   cd rust
   cargo test --all
   ```

5. **Commit your changes:**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

   Use conventional commits:
   - `feat:` - New features
   - `fix:` - Bug fixes
   - `docs:` - Documentation changes
   - `test:` - Adding or updating tests
   - `perf:` - Performance improvements
   - `refactor:` - Code refactoring

6. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request** on GitHub

## Areas Where We Need Help

### High Priority

1. **Implementing Pāṇinian Rules**
   - Currently: 2000+ rules implemented
   - Goal: 4000 rules
   - Files: `rust/vedyut-prakriya/src/`
   - Knowledge needed: Aṣṭādhyāyī, Sanskrit grammar

2. **Improving Segmentation Accuracy**
   - Current: ~85% accuracy
   - Goal: 95%+ accuracy
   - Files: `rust/vedyut-cheda/src/`
   - Knowledge needed: Sandhi rules, disambiguation

3. **Vedic Sanskrit Support**
   - Accents (udātta, anudātta, svarita)
   - Vedic sandhi rules
   - Special dhātus and pratyayas

### Medium Priority

4. **ML Models for Disambiguation**
   - Neural models for choosing best segmentation
   - Part-of-speech tagging
   - Context-aware analysis

5. **Performance Optimization**
   - Target: <50ms per verse
   - Profiling and benchmarking
   - Memory optimization

6. **Documentation & Tutorials**
   - API examples
   - Jupyter notebooks
   - Video tutorials
   - Grammar explanations

### Good First Issues

7. **Testing**
   - Add test cases for edge cases
   - Integration tests
   - Benchmark tests

8. **CLI Tool**
   - Command-line interface for batch processing
   - Interactive mode

9. **Web UI**
   - React frontend
   - WASM integration
   - Interactive examples

## Code Style

### Python

- Follow PEP 8
- Use type hints
- Use `ruff` for formatting and linting
- Maximum line length: 100 characters

Example:
```python
def transliterate(text: str, from_scheme: str, to_scheme: str) -> str:
    """Transliterate text between schemes.
    
    Args:
        text: Input text
        from_scheme: Source script
        to_scheme: Target script
    
    Returns:
        Transliterated text
    """
    # Implementation
    pass
```

### Rust

- Follow Rust style guide
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write doc comments for public APIs

Example:
```rust
/// Transliterate text from one scheme to another
///
/// # Arguments
/// * `text` - The input text
/// * `from` - Source scheme
/// * `to` - Target scheme
///
/// # Returns
/// Transliterated text
pub fn transliterate(text: &str, from: Scheme, to: Scheme) -> String {
    // Implementation
    String::new()
}
```

## Testing Guidelines

### Python Tests

- Use `pytest` for testing
- Aim for >80% code coverage
- Test both success and error cases
- Use `pytest-asyncio` for async tests

```python
def test_transliterate_iast_to_devanagari():
    result = transliterate("dharmakṣetre", "iast", "devanagari")
    assert result == "धर्मक्षेत्रे"
```

### Rust Tests

- Write unit tests in same file
- Write integration tests in `tests/` directory
- Use `criterion` for benchmarks

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transliterate() {
        let result = transliterate("dharmakSetre", Scheme::Slp1, Scheme::Devanagari);
        assert_eq!(result, "धर्मक्षेत्रे");
    }
}
```

## Documentation

- Update README.md for user-facing changes
- Write docstrings for all public APIs
- Add examples in doc comments
- Update API documentation in `docs/`

## Pull Request Process

1. **Ensure all tests pass** locally
2. **Update documentation** if needed
3. **Add tests** for new features
4. **Keep PRs focused** - one feature/fix per PR
5. **Write clear commit messages**
6. **Reference issues** in PR description (e.g., "Fixes #123")

### PR Template

```markdown
## Description
Brief description of changes

## Related Issues
Fixes #123

## Changes
- Added X
- Fixed Y
- Updated Z

## Testing
- [ ] Added unit tests
- [ ] Added integration tests
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Tests pass locally
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (if applicable)
```

## Community

- **Discord**: [Join #vedyut channel](https://discord.gg/7rGdTyWY7Z)
- **GitHub Discussions**: For questions and ideas
- **Mailing List**: [vedyut-discuss](https://groups.google.com/g/vedyut-discuss)

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Help others learn and grow

## Questions?

Feel free to:
- Open an issue for questions
- Join our Discord server
- Email the maintainers

Thank you for contributing to Vedyut! 🙏
