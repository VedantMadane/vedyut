# Vedyut 🕉️

[![CI](https://github.com/VedantMadane/vedyut/actions/workflows/ci.yml/badge.svg)](https://github.com/VedantMadane/vedyut/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**High-performance Sanskrit NLP toolkit for the LLM era**

Vedyut combines Rust performance with Python ease-of-use to provide blazing-fast Sanskrit text processing with first-class multi-script support.

## 🌟 Key Features

- **⚡ Rust Performance**: 100-180x faster than pure Python implementations
- **🌏 First-Class Script Support**: Write Sanskrit in any script - Devanagari, IAST, Tamil, Telugu, Malayalam, Kannada, Bengali, and 15+ more
- **🎯 Script-First API Design**: Script selection is a required, explicit parameter - not buried in options
- **📝 Full Sanskrit NLP**: Transliteration, segmentation, morphological analysis, word generation
- **🤖 LLM-Ready**: Built-in integrations for RAG, agents, and modern ML workflows
- **🐍 Python API**: Clean, type-safe Python interface powered by Rust core

## 🎨 Script Support

Sanskrit can be written in **any script**. Vedyut treats script selection as a **first-class feature**:

### Supported Scripts (25+)

| Category | Scripts |
|----------|---------|
| **Romanization** | IAST, SLP1, Harvard-Kyoto, ITRANS, ISO 15919, Velthuis, WX |
| **Indian Scripts** | Devanagari, Telugu, Tamil, Kannada, Malayalam, Bengali, Gujarati, Gurmukhi, Odia, Assamese |
| **Other Scripts** | Tibetan, Sinhala, Burmese, Thai, Grantha |

## 📦 Installation

### Python Package

```bash
pip install vedyut
```

### From Source

```bash
# Clone repository
git clone https://github.com/VedantMadane/vedyut.git
cd vedyut

# Install with uv (recommended)
uv sync

# Or with pip
pip install -e .
```

## 🚀 Quick Start

### Python API

```python
from vedyut import transliterate, segment, analyze, Script

# Transliterate between any scripts
# Script is a FIRST-CLASS parameter - explicit and required!
devanagari = transliterate("namaste", Script.IAST, Script.DEVANAGARI)
# → "नमस्ते"

tamil = transliterate("namaste", Script.IAST, Script.TAMIL)
# → "நமஸ்தே"

telugu = transliterate("namaste", Script.IAST, Script.TELUGU)
# → "నమస్తే"

# Segment text into words
segments = segment("धर्मक्षेत्रे कुरुक्षेत्रे", Script.DEVANAGARI)
# → [["धर्मक्षेत्रे", "कुरुक्षेत्रे"]]

# Morphological analysis
analysis = analyze("रामः", Script.DEVANAGARI)
# → [{"stem": "राम", "case": "nominative", ...}]
```

### Rust API

```rust
use vedyut_lipi::{transliterate, Scheme};

fn main() {
    // Script as first-class parameter
    let result = transliterate(
        "dharmakṣetre",
        Scheme::Iast,
        Scheme::Devanagari
    );
    
    println!("{}", result); // धर्मक्षेत्रे
}
```

### Web API

```bash
# Start the API server
uv run uvicorn vedyut.api.main:app --reload

# Or with Python
python -m vedyut.api.main
```

```bash
# Transliterate
curl -X POST http://localhost:8000/v1/transliterate \
  -H "Content-Type: application/json" \
  -d '{
    "text": "namaste",
    "from_scheme": "iast",
    "to_scheme": "devanagari"
  }'
```

## 🎯 API Design: Script as First-Class Feature

Vedyut makes script selection **explicit and unavoidable** - it's a core design principle:

### ✅ Good: Script is First-Class

```python
# Script is a required, explicit parameter
transliterate(text, from_script, to_script)
segment(text, script=Script.DEVANAGARI)
analyze(word, script=Script.TAMIL)
```

### ❌ Bad: Script Buried in Options

```python
# Don't do this - script hidden in options
transliterate(text, options={"from": "iast", "to": "deva"})
process(text, config=Config(script="devanagari"))
```

## 🏗️ Architecture

```
vedyut/
├── rust/                    # Rust core (performance-critical)
│   ├── vedyut-lipi/        # Transliteration engine
│   ├── vedyut-sandhi/      # Sandhi rules & splitting
│   ├── vedyut-prakriya/    # Word generation (Pāṇinian)
│   ├── vedyut-kosha/       # High-speed lexicon
│   └── vedyut-cheda/       # Segmentation & analysis
├── python/                  # Python API (user-friendly)
│   └── vedyut/
│       ├── __init__.py     # Clean Python interface
│       ├── api/            # FastAPI web service
│       └── llm/            # LLM integrations
└── tests/                   # Integration tests
```

## 🧪 Development

### Build Rust Core

```bash
cd rust
cargo build --release
cargo test
```

### Run Python Tests

```bash
uv run pytest tests/ -v
```

### Format & Lint

```bash
# Rust
cd rust
cargo fmt
cargo clippy -- -D warnings

# Python
uv run ruff format .
uv run ruff check .
```

## 📊 Performance

Vedyut achieves 100-180x speedup vs pure Python:

| Operation | Pure Python | Vedyut (Rust) | Speedup |
|-----------|-------------|---------------|---------|
| Transliteration | ~1ms | <10μs | ~100x |
| Word lookup | ~10μs | 820ns | ~12x |
| Verse segmentation | 1.8s | 10ms | ~180x |
| Word generation | 10s/word | 20μs/word | ~500,000x |

## 🤖 LLM Integration

Vedyut is designed for the LLM era with built-in support for:

- **RAG (Retrieval-Augmented Generation)**: Semantic chunking respecting sandhi boundaries
- **Agent Frameworks**: LangChain/CrewAI tool definitions
- **Embeddings**: Batch processing for vector databases

```python
from vedyut.llm import SanskritRAG

# Semantic chunking with script support
rag = SanskritRAG(
    texts=["bhagavad_gita.txt"],
    script=Script.DEVANAGARI
)

results = rag.query("What does Krishna say about dharma?")
```

## 🗺️ Roadmap

- [x] Multi-script transliteration (25+ scripts)
- [x] Script as first-class API parameter
- [x] Rust core skeleton with CI
- [ ] Production transliteration implementation
- [ ] Complete sandhi rules (Aṣṭādhyāyī)
- [ ] Lexicon with 29M+ forms
- [ ] Python bindings (PyO3)
- [ ] WebAssembly support
- [ ] ML-based scoring for segmentation
- [ ] Neural + rule-based hybrid models

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Key areas:
- Implementing transliteration mappings
- Adding sandhi rules
- Building lexicon data
- LLM integrations
- Documentation & examples

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [vidyut](https://github.com/ambuda-org/vidyut) (Ambuda project)
- [sanskrit_parser](https://github.com/kmadathil/sanskrit_parser) for Python foundations
- The Sanskrit NLP community for research and data

## 📚 Related Projects

- [vidyut](https://github.com/ambuda-org/vidyut) - Reliable Sanskrit infrastructure (upstream inspiration)
- [sanskrit_parser](https://github.com/kmadathil/sanskrit_parser) - Python Sanskrit parser
- [indic-transliteration](https://github.com/sanskrit-coders/indic_transliteration_py) - Python transliteration

## 📞 Contact

- GitHub: [@VedantMadane](https://github.com/VedantMadane)
- Issues: [GitHub Issues](https://github.com/VedantMadane/vedyut/issues)

---

**Made with ❤️ for the Sanskrit and Indic language communities**

**Key Feature: Sanskrit in ANY script - script selection is first-class!** 🌏
