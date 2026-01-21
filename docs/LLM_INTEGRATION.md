# LLM Integration Guide

## Overview

Vedyut's LLM integration serves **two purposes**:

1. **Development Assistant**: Use LLMs + RAG to help implement grammar rules
2. **Runtime Features**: Use LLMs for disambiguation, translation, explanations

---

## Can LLMs Understand Sanskrit Grammar Treatises?

### ✅ What LLMs CAN Do

- **Read Aṣṭādhyāyī sūtras** (Sanskrit, Devanagari, or transliterated)
- **Understand English textbooks** (Kale, Whitney, etc.)
- **Parse grammatical terminology** (dhātu, lakāra, sandhi, vibhakti)
- **Explain rules** in natural language
- **Generate code stubs** from rule descriptions
- **Suggest test cases** based on grammar
- **Cross-reference** multiple sources (Pāṇini + Kāśikā + modern)

### ❌ What LLMs CANNOT Do Reliably

- **100% accuracy** (probabilistic, not deterministic)
- **Complex rule ordering** (adhikāra, anuvṛtti, asiddha, tripādī)
- **All edge cases** (rare combinations, exceptions)
- **Consistency** (same query → different answers)
- **No hallucination** (may invent plausible but wrong rules)

### 🎯 Best Practice: LLM as Assistant, Not Autonomous

**Use LLMs to:**
- Retrieve relevant sūtras from texts (RAG)
- Generate initial code (YOU review & refine)
- Explain rules for documentation
- Suggest test cases

**DO NOT use LLMs to:**
- Generate production code without review
- Make grammar decisions without expert validation
- Replace formal grammar implementation

---

## Setup

### 1. Install Dependencies

```bash
uv add --optional llm litellm numpy openai anthropic
```

### 2. Set API Keys

```bash
# .env
VEDYUT_LLM_MODEL=gpt-4o  # or claude-3-5-sonnet-20241022, gemini/gemini-1.5-pro
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
GOOGLE_API_KEY=...
```

### 3. Prepare Grammar Texts

Create `data/grammar/` with your texts:

```
data/grammar/
├── ashtadhyayi.txt           # Pāṇini's sūtras (Sanskrit)
├── kashika.txt               # Kāśikā commentary (Sanskrit)
├── kale_higher_grammar.txt   # M.R. Kale textbook (English)
├── whitney_grammar.txt       # Whitney's grammar (English)
└── custom_rules.json         # Your structured rules
```

**Example JSON format:**
```json
[
  {
    "sutra": "1.1.1",
    "sanskrit": "वृद्धिरादैच्",
    "transliteration": "vṛddhir ādaic",
    "english": "The sounds a, ai, au are called vṛddhi",
    "explanation": "This sūtra defines which vowels fall under the technical term vṛddhi...",
    "topic": "sandhi",
    "applies_to": ["vowel_sandhi", "suffix_vowels"]
  }
]
```

---

## Usage

### Basic LLM Client

```python
from vedyut.llm import LLMClient

# Initialize (uses VEDYUT_LLM_MODEL env var or default)
llm = LLMClient()

# Simple completion
response = llm.complete([
    {"role": "user", "content": "Explain sandhi rule: a + i → e"}
])
print(response)

# JSON mode
result = llm.complete_with_json([
    {"role": "user", "content": "List 3 Sanskrit dhātus in JSON with meanings"}
])
print(result)  # Parsed JSON dict

# Embeddings
embeddings = llm.embed(["धर्मः", "अर्थः", "कामः"])
print(len(embeddings))  # 3
print(len(embeddings[0]))  # 3072 (for text-embedding-3-large)
```

### Switch Models (No Code Changes)

```bash
# OpenAI GPT-4o
export VEDYUT_LLM_MODEL=gpt-4o

# Anthropic Claude
export VEDYUT_LLM_MODEL=claude-3-5-sonnet-20241022

# Google Gemini
export VEDYUT_LLM_MODEL=gemini/gemini-1.5-pro

# Azure OpenAI
export VEDYUT_LLM_MODEL=azure/gpt-4o

# Local Ollama
export VEDYUT_LLM_MODEL=ollama/llama3.1
```

---

## RAG for Grammar Treatises

### Load and Index Grammar Texts

```python
from vedyut.llm import GrammarRAG

# Initialize
rag = GrammarRAG(data_dir="data/grammar")

# Load texts
rag.load_texts()  # Loads .txt and .json files

# Build embedding index (run once, then cached)
rag.build_index()  # Generates embeddings, saves to grammar_index.json
```

### Query Grammar Rules

```python
# Find relevant sūtras
results = rag.query("How to form present tense verbs?", top_k=3)

for chunk, score in results:
    print(f"[{chunk.source} {chunk.sutra_number}] (score: {score:.3f})")
    print(chunk.text[:200])
    print()

# Filter by topic
sandhi_rules = rag.query("vowel combinations", top_k=5, topic="sandhi")

# Filter by language
english_only = rag.query("case endings", top_k=3, language="english")
```

### Generate Code from Rules

```python
# Generate Rust implementation
code = rag.generate_code(
    task_description="Implement sandhi rule: a + i → e",
    language="rust"
)
print(code)

# Output:
# /// Apply sandhi rule: a + i → e (Pāṇini 6.1.87)
# ///
# /// When short or long 'a' is followed by 'i' or 'ī',
# /// they combine to form 'e'.
# pub fn sandhi_a_i_to_e(left: &str, right: &str) -> Option<String> {
#     if left.ends_with('a') || left.ends_with('A') {
#         if right.starts_with('i') || right.starts_with('I') {
#             let left_stem = &left[..left.len()-1];
#             let right_rest = &right[1..];
#             return Some(format!("{}e{}", left_stem, right_rest));
#         }
#     }
#     None
# }
```

### Explain Grammar Rules

```python
# Explain specific sūtra
explanation = rag.explain_rule(sutra_number="1.1.1")
print(explanation)

# Query-based explanation
explanation = rag.explain_rule(query="What is sandhi?")
print(explanation)
```

---

## Sanskrit-Specific Tasks

### Disambiguate Segmentation

```python
from vedyut.llm import disambiguate_segmentation, LLMClient

text = "रामोवाच"
candidates = [
    ["रामः", "उवाच"],  # Rama said
    ["राम", "ओवाच"],   # Invalid
]

llm = LLMClient()
best_idx = disambiguate_segmentation(text, candidates, llm)
print(f"Best: {candidates[best_idx]}")  # ["रामः", "उवाच"]
```

### Translation

```python
from vedyut.llm import translate_sanskrit

# Simple translation
translation = translate_sanskrit("धर्मक्षेत्रे कुरुक्षेत्रे")
print(translation)

# With word-by-word breakdown
detailed = translate_sanskrit(
    "धर्मक्षेत्रे कुरुक्षेत्रे",
    with_explanation=True
)
print(detailed)
```

### Grammar Explanation

```python
from vedyut.llm import explain_grammar

# With analysis dict
analysis = {
    "lemma": "राम",
    "case": "nominative",
    "number": "singular",
    "gender": "masculine"
}

explanation = explain_grammar(word="रामः", analysis=analysis)
print(explanation)
# Output: "रामः is the nominative singular form of राम (Rama)..."
```

---

## Development Workflow: RAG-Assisted Implementation

### Step 1: Query Relevant Rules

```python
from vedyut.llm import GrammarRAG

rag = GrammarRAG(data_dir="data/grammar")
rag.load_texts()
rag.build_index()

# Find rules for feature you're implementing
results = rag.query("aorist tense formation rules", top_k=5)

for chunk, score in results:
    print(f"\n=== {chunk.source} {chunk.sutra_number} (score: {score:.3f}) ===")
    print(chunk.text)
```

### Step 2: Generate Code Stub

```python
code = rag.generate_code(
    rule_description="Implement aorist (luṅ) formation for bhvādi dhātus",
    language="rust",
    include_tests=True
)

# Save to file for review
with open("rust/vedyut-prakriya/src/aorist.rs", "w") as f:
    f.write(code)
```

### Step 3: Review & Refine

**YOU must:**
1. Read the generated code carefully
2. Verify against grammar sources
3. Add edge cases the LLM missed
4. Test extensively
5. Consult experts if uncertain

### Step 4: Generate Test Cases

```python
from vedyut.llm import generate_test_cases

tests = generate_test_cases(
    function_description="aorist (luṅ) formation for 'भू' dhatu",
    rag=rag,
    num_cases=20
)

# Convert to Rust tests
for test in tests:
    print(f"""
#[test]
fn test_{test['description'].lower().replace(' ', '_')}() {{
    let result = form_aorist("{test['input']}");
    assert_eq!(result, "{test['expected']}");
}}
""")
```

---

## Configuration

### Model Selection Guide

| Model | Best For | Cost | Speed |
|-------|----------|------|-------|
| **gpt-4o** | General Sanskrit tasks | $$ | Fast |
| **gpt-4-turbo** | Complex grammar analysis | $$$ | Medium |
| **claude-3-5-sonnet** | Code generation, reasoning | $$ | Medium |
| **gemini-1.5-pro** | Long context (treatises) | $$ | Fast |
| **gemini-1.5-flash** | Quick queries, cost-effective | $ | Very Fast |

**Recommendation:** Start with `gpt-4o` or `claude-3-5-sonnet-20241022`.

### Environment Variables

```bash
# Required
OPENAI_API_KEY=sk-...          # If using OpenAI models
ANTHROPIC_API_KEY=sk-ant-...   # If using Anthropic models
GOOGLE_API_KEY=...             # If using Google models

# Optional
VEDYUT_LLM_MODEL=gpt-4o                    # Default model
VEDYUT_EMBEDDING_MODEL=text-embedding-3-large  # Embedding model
VEDYUT_LLM_TEMPERATURE=0.7                 # Sampling temperature
```

---

## Best Practices

### ⚠️ Critical Warnings

1. **Never blindly trust LLM-generated grammar code**
   - LLMs can hallucinate rules
   - They miss edge cases
   - They may misinterpret sūtra order

2. **Always validate against sources**
   - Cross-check with Aṣṭādhyāyī
   - Consult commentaries (Kāśikā, Mahābhāṣya)
   - Test with known-correct examples

3. **Use expert review**
   - Post to sanskrit-programmers mailing list
   - Ask on Ambuda Discord (#vidyut channel)
   - Consult with @akprasad, @neeleshb

### ✅ Good Use Cases

1. **Code stub generation** → human refinement
2. **Test case generation** → human validation
3. **Documentation** (explaining existing rules)
4. **Research** (finding relevant sūtras quickly)
5. **Disambiguation** (choosing between valid alternatives)

### ❌ Bad Use Cases

1. **Autonomous grammar implementation** (too error-prone)
2. **Production code without testing** (dangerous)
3. **Complex rule ordering** (LLMs struggle here)

---

## Example: Full Development Workflow

```python
from vedyut.llm import GrammarRAG, suggest_implementation, generate_test_cases

# 1. Setup RAG with your grammar texts
rag = GrammarRAG(data_dir="data/grammar")
rag.load_texts()
rag.build_index()

# 2. Research: Find relevant rules
results = rag.query("How does guṇa sandhi work?", top_k=3)
print("Found rules:")
for chunk, score in results:
    print(f"  {chunk.sutra_number}: {chunk.text[:100]}...")

# 3. Generate implementation (STARTING POINT, not final code!)
code = suggest_implementation(
    rule_description="Implement guṇa sandhi: a/ā + i/ī → e",
    rag=rag,
    language="rust",
    include_tests=True
)

print("Generated code (REVIEW REQUIRED!):")
print(code)

# 4. Generate comprehensive test cases
tests = generate_test_cases(
    function_description="guṇa sandhi for a/ā + i/ī",
    rag=rag,
    num_cases=15
)

print(f"\nGenerated {len(tests)} test cases:")
for test in tests[:3]:
    print(f"  Input: {test['input']} → {test['expected']} ({test['description']})")

# 5. YOU: Review code, add edge cases, test, refine, commit
```

---

## Feeding Grammar Texts to LLM

### What to Provide

**Sanskrit Sources (Primary):**
- Aṣṭādhyāyī (8 chapters, ~4000 sūtras)
- Kāśikā Vṛtti (commentary explaining sūtras)
- Siddhānta Kaumudī (rearranged for learning)
- Mahābhāṣya (Patañjali's detailed analysis)

**English Sources (Reference):**
- M.R. Kale: "Higher Sanskrit Grammar"
- Whitney: "Sanskrit Grammar"
- Coulson: "Teach Yourself Sanskrit"
- Modern linguistics papers

### How LLM Will Process Them

**Via RAG:**
1. Chunk texts into manageable pieces (~500-1000 chars)
2. Embed each chunk (semantic vector)
3. Store in vector database
4. When you ask a question:
   - Embed the question
   - Retrieve top-K most similar chunks
   - Feed to LLM as context
   - LLM generates answer using retrieved rules

**Example:**

```
You ask: "How to implement aorist tense?"

RAG retrieves:
  [Aṣṭādhyāyī 3.2.110] "luṅ"
  [Kāśikā] "luṅ lakāraḥ bhūte kriyāyāṃ..."
  [Kale pg 234] "The aorist (luṅ) expresses past action..."

LLM reads these chunks + your question → generates:
  - Rust code stub
  - Explanation
  - Test cases
  
YOU review and refine!
```

---

## Performance & Cost

### Embedding Costs (One-Time)

Assuming 1000 pages of grammar text (~500K words):

| Provider | Model | Cost | Time |
|----------|-------|------|------|
| OpenAI | text-embedding-3-large | ~$0.50 | 2-3 min |
| OpenAI | text-embedding-3-small | ~$0.10 | 1-2 min |

### Query Costs (Per Development Session)

Typical session: 10-20 queries with retrieved context

| Provider | Model | Cost/Session |
|----------|-------|--------------|
| OpenAI | gpt-4o | $0.20-0.50 |
| Anthropic | claude-3-5-sonnet | $0.30-0.70 |
| Google | gemini-1.5-pro | $0.10-0.30 |
| Google | gemini-1.5-flash | $0.02-0.05 |

**Total cost to implement 100 rules with LLM assistance: ~$10-30**

---

## Limitations & Mitigations

### Limitation 1: LLM May Misinterpret Sūtra Order

**Problem:** Aṣṭādhyāyī has complex precedence rules (पूर्वपरनित्यान्तरङ्गापवादानाम् उत्तरोत्तरं बलीयः)

**Mitigation:**
- Use RAG to retrieve **multiple sūtras** with context
- Cross-reference with Kāśikā commentary
- Implement rule-ordering logic in Rust (deterministic)
- LLM only suggests, YOU verify order

### Limitation 2: Hallucination

**Problem:** LLM may invent plausible-sounding but incorrect sūtras

**Mitigation:**
- Always include source attribution in RAG context
- Verify sūtra numbers against original text
- Use multiple LLMs (GPT-4 + Claude) for cross-validation
- Maintain test suite with known-correct examples

### Limitation 3: Incomplete Coverage

**Problem:** English textbooks simplify; Sanskrit texts are terse

**Mitigation:**
- Load **both** Sanskrit originals AND English explanations
- RAG retrieves from both → LLM synthesizes
- For ambiguous cases, consult experts

---

## Advanced: Multi-LLM Validation

Use multiple LLMs to validate implementations:

```python
def validate_with_multiple_llms(code: str, rule: str, rag: GrammarRAG):
    """Cross-validate code with GPT-4, Claude, and Gemini"""
    
    models = ["gpt-4o", "claude-3-5-sonnet-20241022", "gemini/gemini-1.5-pro"]
    results = []
    
    for model in models:
        llm = LLMClient(model=model)
        rag_with_llm = GrammarRAG(llm_client=llm)
        rag_with_llm.chunks = rag.chunks  # Share chunks
        
        validation = validate_rule_implementation(code, rule, rag_with_llm)
        results.append({
            "model": model,
            "is_valid": validation["is_valid"],
            "confidence": validation["confidence"],
            "issues": validation["issues"]
        })
    
    # Consensus: if 2+ models agree it's valid
    valid_count = sum(1 for r in results if r["is_valid"])
    return valid_count >= 2, results
```

---

## Summary

### ✅ Use LLMs For:
- **RAG-based rule lookup** from grammar treatises
- **Code stub generation** (with your review)
- **Test case generation**
- **Documentation & explanations**
- **Disambiguation** of valid alternatives

### ❌ Don't Use LLMs For:
- **Production code** without expert review
- **Complex rule ordering** (implement deterministically)
- **Ground truth** without validation

### 🎯 Workflow:
1. Load grammar texts (Sanskrit + English)
2. Build RAG index (one-time)
3. Query for relevant rules
4. LLM generates code stub
5. **YOU review, test, refine** ← Critical step!
6. Consult experts for complex cases

**LLMs are powerful assistants for Sanskrit NLP development, but human expertise remains essential for correctness.**
