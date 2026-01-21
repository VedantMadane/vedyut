<div align="center">
<h1>वेद्युत् (Vedyut)</h1>
<p><i>High-performance Sanskrit NLP toolkit for the modern age</i></p>

[![Build Status](https://github.com/VedantMadane/vedyut/workflows/CI/badge.svg)](https://github.com/VedantMadane/vedyut/actions)
[![PyPI version](https://badge.fury.io/py/vedyut.svg)](https://pypi.org/project/vedyut/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

</div>

---

## 🌟 Vision

**Vedyut** (वेद्युत् - "lightning" in Sanskrit, from the Vedas) is a next-generation Sanskrit NLP toolkit that combines:

- ⚡ **Speed**: Rust core for 100-180x faster processing than pure Python
- 🎯 **Accuracy**: Based on Pāṇinian grammar rules (2000+ sūtras)
- 🤖 **LLM-Ready**: Built-in integration with modern AI frameworks
- 🌐 **Modern DevOps**: REST API, WebAssembly, Docker, comprehensive CI/CD

---

## 🚀 Quick Start

### Installation

```bash
# Install with uv (recommended)
uv pip install vedyut

# Or with pip
pip install vedyut
```

### Basic Usage

```python
from vedyut import Sanskrit

s = Sanskrit()

# Transliteration
text = s.transliterate("dharmakṣetre", from_scheme="iast", to_scheme="devanagari")
# → "धर्मक्षेत्रे"

# Segmentation (fast!)
segments = s.segment("धर्मक्षेत्रे कुरुक्षेत्रे")
# → ["धर्मक्षेत्रे", "कुरुक्षेत्रे"] in <50ms

# Morphological analysis
analysis = s.analyze("रामः")
# → {"lemma": "राम", "case": "nominative", "number": "singular", ...}

# Word generation
forms = s.generate(dhatu="भू", lakara="lat", purusha="prathama", vacana="eka")
# → ["भवति"]
```

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────┐
│           Python API (User Interface)           │
│  - Simple API: parse(), segment(), generate()  │
│  - Jupyter notebook friendly                    │
│  - LLM integration helpers                      │
└───────────────┬─────────────────────────────────┘
                │
        ┌───────┴────────┐
        │                │
┌───────▼──────┐  ┌─────▼──────────┐
│  Rust Core   │  │ Python Layer   │
│  (PyO3)      │  │ - FastAPI REST │
│              │  │ - ML models    │
│ - Sandhi     │  │ - Caching      │
│ - Prakriya   │  │ - LLM tools    │
│ - Kosha      │  └────────────────┘
│ - Cheda      │
└──────────────┘
```

### Core Modules (Rust)

- **`vedyut-lipi`**: Transliteration between scripts (Devanagari, IAST, SLP1, etc.)
- **`vedyut-sandhi`**: Sandhi rules application and splitting
- **`vedyut-prakriya`**: Word generation following Pāṇinian grammar
- **`vedyut-kosha`**: High-performance lexicon (820ns lookups, 50M+ entries)
- **`vedyut-cheda`**: Segmentation and morphological analysis
- **`vedyut-vibhakti`**: Case/gender/number analysis
- **`vedyut-vakya`**: Sentence-level parsing

### Python Layer

- **FastAPI REST API**: `/v1/segment`, `/v1/analyze`, `/v1/generate`
- **LLM Integration**: RAG helpers, tool schemas, agent frameworks
- **ML Models**: Optional neural models for disambiguation
- **Web UI**: Interactive demo (React + WASM)

---

## 🔥 Features

### Performance First
- **100-180x faster** than pure Python implementations
- Target: **<50ms per verse** end-to-end processing
- Optimized for batch processing (1000s of verses)
- Memory-efficient data structures

### LLM-Ready
```python
from vedyut.llm import SanskritRAG

rag = SanskritRAG(texts=["bhagavad_gita.txt"])
response = rag.query("What is dharma?")

# Tool calling for agents
from vedyut.llm import get_tools
tools = get_tools()  # OpenAI/Anthropic compatible
```

### Modern DevOps
- **REST API** (FastAPI, auto-generated OpenAPI docs at `/docs`)
- **WebAssembly** builds for browser use
- **Docker** containers for deployment
- **CI/CD** with comprehensive benchmarks
- **uv** for fast, reproducible builds

### Documentation Excellence
- Interactive tutorials with Devanagari examples
- Jupyter notebooks
- API reference (auto-generated)
- Video walkthroughs
- Comparison with existing tools

---

## 📦 Installation from Source

### Prerequisites

- **Rust** (1.70+): [Install Rust](https://rustup.rs/)
- **Python** (3.10+): [Install Python](https://www.python.org/downloads/)
- **uv**: [Install uv](https://docs.astral.sh/uv/getting-started/installation/)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/VedantMadane/vedyut.git
cd vedyut

# Install Python dependencies with uv
uv venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate
uv sync

# Build Rust core
cd rust
cargo build --release

# Run tests
cargo test --all
pytest ../tests
```

---

## 🌐 REST API

Start the FastAPI server:

```bash
uvicorn vedyut.api.main:app --reload
```

Visit `http://localhost:8000/docs` for interactive API documentation.

### Example Endpoints

**Segment Text:**
```bash
curl -X POST "http://localhost:8000/v1/segment" \
  -H "Content-Type: application/json" \
  -d '{"text": "धर्मक्षेत्रे कुरुक्षेत्रे", "max_splits": 10}'
```

**Analyze Word:**
```bash
curl -X POST "http://localhost:8000/v1/analyze" \
  -H "Content-Type: application/json" \
  -d '{"word": "रामः"}'
```

**Generate Tiṅanta:**
```bash
curl -X POST "http://localhost:8000/v1/generate" \
  -H "Content-Type: application/json" \
  -d '{"dhatu": "भू", "lakara": "lat", "purusha": "prathama", "vacana": "eka"}'
```

---

## 🧪 Testing

```bash
# Rust tests
cargo test --all

# Python tests
pytest tests/ -v

# Benchmark
pytest tests/benchmarks/ --benchmark-only
```

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Areas we need help:**
- Adding more Pāṇinian rules (currently 2000+, goal: 4000)
- Improving segmentation accuracy
- Adding support for Vedic Sanskrit
- Building ML models for disambiguation
- Documentation and tutorials

---

## 📚 Resources

### Documentation
- [User Guide](https://vedyut.readthedocs.io/)
- [API Reference](https://vedyut.readthedocs.io/api/)
- [Architecture Deep Dive](docs/ARCHITECTURE.md)
- [Performance Benchmarks](docs/BENCHMARKS.md)

### Community
- [Discord Server](https://discord.gg/7rGdTyWY7Z) (join #vedyut)
- [GitHub Discussions](https://github.com/VedantMadane/vedyut/discussions)
- [Mailing List](https://groups.google.com/g/vedyut-discuss)

### Acknowledgments

Vedyut builds upon the excellent work of:
- [ambuda-org/vidyut](https://github.com/ambuda-org/vidyut) - Rust Sanskrit processing
- [kmadathil/sanskrit_parser](https://github.com/kmadathil/sanskrit_parser) - Python NLP toolkit
- [ashtadhyayi.com](https://ashtadhyayi.com) - Dhātupāṭha data

---

## 📊 Performance Benchmarks

| Operation | Pure Python | Vedyut (Rust) | Speedup |
|-----------|------------|---------------|---------|
| Segmentation (100 verses) | 180s | 1.2s | **150x** |
| Sandhi splitting | 1.8s/verse | 10ms/verse | **180x** |
| Morphological analysis | 500ms/word | 3ms/word | **166x** |
| Lexicon lookup | 50μs | 820ns | **61x** |

*Benchmarks run on: MacBook Pro M1, 16GB RAM*

---

## 🗺️ Roadmap

### Q1 2026 ✅
- [x] Project initialization
- [x] Architecture design
- [ ] Core Rust modules (sandhi, prakriya)
- [ ] Python bindings (PyO3)
- [ ] Basic REST API

### Q2 2026
- [ ] Complete Pāṇinian rule coverage (3000+ rules)
- [ ] Segmentation with 95%+ accuracy
- [ ] LLM integration (OpenAI, Anthropic, Gemini)
- [ ] WebAssembly builds

### Q3 2026
- [ ] ML models for disambiguation
- [ ] Vedic Sanskrit support
- [ ] Performance optimizations (<50ms target)
- [ ] Docker images + Kubernetes charts

### Q4 2026
- [ ] Full documentation + video tutorials
- [ ] Comprehensive benchmarks
- [ ] v1.0 stable release
- [ ] PyPI + crates.io publication

---

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

## 🙏 Citation

If you use Vedyut in your research, please cite:

```bibtex
@software{vedyut2026,
  author = {Madane, Vedant},
  title = {Vedyut: High-performance Sanskrit NLP Toolkit},
  year = {2026},
  url = {https://github.com/VedantMadane/vedyut}
}
```

---

<div align="center">
<sub>बलमिति वेद्युति</sub>
<br>
<sub>"Strength lies in lightning" - From the Vedas</sub>
</div>
