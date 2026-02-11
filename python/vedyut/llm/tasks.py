"""Sanskrit-specific LLM tasks using RAG"""

from typing import List, Dict, Optional
from .client import LLMClient
from .rag import GrammarRAG, GrammarChunk


def disambiguate_segmentation(
    text: str,
    candidates: List[List[str]],
    llm: Optional[LLMClient] = None,
    rag: Optional[GrammarRAG] = None,
) -> int:
    """Use LLM to choose best segmentation from candidates

    Args:
        text: Original Sanskrit text
        candidates: List of possible segmentations (each a list of words)
        llm: LLM client (created if None)
        rag: Grammar RAG (optional, for rule-based context)

    Returns:
        Index of best candidate (0-indexed)

    Example:
        >>> text = "धर्मक्षेत्रे"
        >>> candidates = [
        ...     ["धर्म", "क्षेत्रे"],
        ...     ["धर्मक्षेत्रे"],
        ... ]
        >>> best_idx = disambiguate_segmentation(text, candidates)
        >>> print(candidates[best_idx])
    """
    if llm is None:
        llm = LLMClient()

    # Build context from sandhi rules if RAG available
    context = ""
    if rag:
        results = rag.query(f"sandhi rules for: {text}", top_k=2, topic="sandhi")
        if results:
            context = "\n\nRelevant sandhi rules:\n" + "\n".join(
                [f"- {chunk.text[:200]}..." for chunk, _ in results]
            )

    candidates_text = "\n".join([f"{i + 1}. {' + '.join(seg)}" for i, seg in enumerate(candidates)])

    prompt = f"""You are a Sanskrit grammar expert. Given a Sanskrit text and multiple possible segmentations, choose the most grammatically correct and semantically meaningful one.

Text: {text}

Possible segmentations:
{candidates_text}
{context}

Respond with ONLY the number (1-{len(candidates)}) of the best segmentation.
Number: """

    response = llm.complete([{"role": "user", "content": prompt}], temperature=0.3, max_tokens=10)

    try:
        number = int(response.strip().split()[0])
        return max(0, min(number - 1, len(candidates) - 1))
    except (ValueError, IndexError):
        return 0  # Default to first candidate


def translate_sanskrit(
    text: str,
    target_lang: str = "english",
    llm: Optional[LLMClient] = None,
    with_explanation: bool = False,
) -> str:
    """Translate Sanskrit text to target language

    Args:
        text: Sanskrit text (Devanagari or transliterated)
        target_lang: Target language (default: "english")
        llm: LLM client
        with_explanation: Include word-by-word breakdown

    Returns:
        Translation (and optional explanation)
    """
    if llm is None:
        llm = LLMClient()

    if with_explanation:
        prompt = f"""Translate this Sanskrit text to {target_lang} with word-by-word explanation:

Sanskrit: {text}

Provide:
1. Word-by-word breakdown with grammatical analysis
2. Smooth {target_lang} translation

FORMAT:
Word-by-word:
- word1 (grammatical info): meaning
- word2 (grammatical info): meaning

Translation: [full translation]
"""
    else:
        prompt = f"Translate this Sanskrit text to {target_lang}: {text}"

    return llm.complete([{"role": "user", "content": prompt}], temperature=0.5)


def explain_grammar(
    word: str,
    analysis: Optional[Dict] = None,
    llm: Optional[LLMClient] = None,
    rag: Optional[GrammarRAG] = None,
) -> str:
    """Generate natural language explanation of grammatical analysis

    Args:
        word: Sanskrit word
        analysis: Grammatical analysis dict (lemma, case, number, etc.)
        llm: LLM client
        rag: Grammar RAG for rule references

    Returns:
        Beginner-friendly explanation
    """
    if llm is None:
        llm = LLMClient()

    analysis_text = ""
    if analysis:
        analysis_text = "\n".join([f"- {k}: {v}" for k, v in analysis.items()])

    # Get relevant grammar rules if RAG available
    context = ""
    if rag and analysis:
        query = f"grammar for {word} "
        if "case" in analysis:
            query += f"case {analysis['case']}"
        if "tense" in analysis:
            query += f"tense {analysis['tense']}"

        results = rag.query(query, top_k=2)
        if results:
            context = "\n\nGrammar rules:\n" + "\n".join(
                [f"[{chunk.source}] {chunk.text[:150]}..." for chunk, _ in results]
            )

    prompt = f"""Explain the grammar of this Sanskrit word in simple, beginner-friendly terms:

Word: {word}

Grammatical analysis:
{analysis_text}
{context}

Provide a clear explanation suitable for someone learning Sanskrit. Include:
1. What the word means
2. Its grammatical function (case, number, gender, tense, etc.)
3. Why it has this form
4. A simple example sentence

EXPLANATION:
"""

    return llm.complete([{"role": "user", "content": prompt}], temperature=0.6)


def suggest_implementation(
    rule_description: str,
    rag: GrammarRAG,
    language: str = "rust",
    include_tests: bool = True,
) -> str:
    """Generate code implementation suggestion from grammar rule

    ⚠️ WARNING: LLM-generated code requires human review!
    Use this as a starting point, not production code.

    Args:
        rule_description: Description of what to implement
        rag: Grammar RAG (required for rule lookup)
        language: Target programming language
        include_tests: Generate test cases

    Returns:
        Generated code with comments
    """
    # Retrieve relevant grammar chunks
    results = rag.query(rule_description, top_k=3)
    context_chunks = [chunk for chunk, _ in results]

    if not context_chunks:
        return f"# No relevant grammar rules found for: {rule_description}"

    context_text = "\n\n".join(
        [f"[{chunk.source} {chunk.sutra_number or ''}]\n{chunk.text}" for chunk in context_chunks]
    )

    test_instruction = ""
    if include_tests:
        test_instruction = "\n4. Test cases with examples"

    prompt = f"""You are a Sanskrit NLP expert implementing Pāṇinian grammar rules in code.

Grammar References:
{context_text}

Task: {rule_description}

Generate clean, production-ready {language} code with:
1. Clear function signature with type annotations
2. Implementation following the grammar rules above
3. Detailed comments explaining each step and referencing sūtras{test_instruction}

⚠️ IMPORTANT:
- Be precise with grammar rules
- Handle edge cases
- Note any ambiguities or limitations

{language.upper()} CODE:
"""

    llm = rag.llm
    return llm.complete([{"role": "user", "content": prompt}], temperature=0.3, max_tokens=2000)


def generate_test_cases(
    function_description: str,
    rag: Optional[GrammarRAG] = None,
    llm: Optional[LLMClient] = None,
    num_cases: int = 10,
) -> List[Dict[str, str]]:
    """Generate test cases for a Sanskrit NLP function

    Args:
        function_description: What the function does
        rag: Grammar RAG for rule-based examples
        llm: LLM client
        num_cases: Number of test cases to generate

    Returns:
        List of {"input": "...", "expected": "...", "description": "..."} dicts
    """
    if llm is None:
        llm = LLMClient()

    # Get grammar context if available
    context = ""
    if rag:
        results = rag.query(function_description, top_k=2)
        if results:
            context = "\n\nGrammar references:\n" + "\n".join(
                [f"{chunk.text[:200]}..." for chunk, _ in results]
            )

    prompt = f"""Generate {num_cases} diverse test cases for this Sanskrit NLP function:

Function: {function_description}
{context}

For each test case, provide:
1. Input (Sanskrit text or word)
2. Expected output
3. Brief description of what it tests

Return as JSON array:
[
  {{
    "input": "...",
    "expected": "...",
    "description": "..."
  }},
  ...
]

JSON:
"""

    try:
        result = llm.complete_with_json([{"role": "user", "content": prompt}])
        if isinstance(result, dict) and "test_cases" in result:
            return result["test_cases"]
        elif isinstance(result, list):
            return result
        else:
            return []
    except Exception as e:
        print(f"Error generating test cases: {e}")
        return []


def validate_rule_implementation(
    code: str,
    rule_description: str,
    rag: GrammarRAG,
    language: str = "rust",
) -> Dict[str, any]:
    """Validate that code correctly implements a grammar rule

    ⚠️ WARNING: This is a heuristic check, not formal verification!
    Always test with actual Sanskrit data.

    Args:
        code: Code to validate
        rule_description: What it should implement
        rag: Grammar RAG for rule lookup
        language: Programming language

    Returns:
        {
            "is_valid": bool,
            "confidence": float (0-1),
            "issues": List[str],
            "suggestions": List[str]
        }
    """
    # Retrieve grammar rules
    results = rag.query(rule_description, top_k=2)
    context_text = "\n\n".join([chunk.text for chunk, _ in results])

    prompt = f"""Review this {language} code implementing a Pāṇinian grammar rule.

Grammar Rule:
{context_text}

Implementation:
```{language}
{code}
```

Task: {rule_description}

Analyze if the code correctly implements the grammar rule. Return JSON:
{{
  "is_valid": true/false,
  "confidence": 0.0-1.0,
  "issues": ["issue 1", "issue 2", ...],
  "suggestions": ["suggestion 1", "suggestion 2", ...]
}}

JSON:
"""

    try:
        return rag.llm.complete_with_json([{"role": "user", "content": prompt}])
    except Exception as e:
        return {
            "is_valid": False,
            "confidence": 0.0,
            "issues": [f"Validation failed: {e}"],
            "suggestions": [],
        }
