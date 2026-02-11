"""Example: Using LLM + RAG to assist with grammar rule implementation

This example shows how to:
1. Load Sanskrit grammar treatises (Aṣṭādhyāyī, English textbooks)
2. Query relevant rules using RAG
3. Generate code implementations with LLM assistance
4. Validate and refine
"""

import os

from vedyut.llm import GrammarRAG, generate_test_cases, suggest_implementation


def main():
    # Check for API key
    if not os.getenv("OPENAI_API_KEY") and not os.getenv("ANTHROPIC_API_KEY"):
        print("⚠️  No API key found. Set OPENAI_API_KEY or ANTHROPIC_API_KEY")
        print("   export OPENAI_API_KEY=sk-...")
        return

    print("=== Vedyut Grammar Assistant ===\n")

    # Initialize RAG with grammar texts
    print("1. Initializing RAG with grammar treatises...")
    rag = GrammarRAG(data_dir="data/grammar")

    # Load texts (Aṣṭādhyāyī, Kāśikā, English textbooks)
    print("2. Loading grammar texts...")
    rag.load_texts()

    if not rag.chunks:
        print("\n⚠️  No grammar texts found in data/grammar/")
        print("   Please add:")
        print("   - ashtadhyayi.txt (Pāṇini's sūtras)")
        print("   - kashika.txt (Commentary)")
        print("   - kale_grammar.txt (English textbook)")
        print("   - custom_rules.json (Your structured rules)")
        return

    # Build search index (generates embeddings)
    print("3. Building search index (this may take a minute)...")
    rag.build_index()

    print(f"\n✅ Ready! Loaded {len(rag.chunks)} grammar chunks\n")

    # Example 1: Query for relevant rules
    print("=" * 60)
    print("Example 1: Query Grammar Rules")
    print("=" * 60)

    query = "How to form present tense verbs from dhātus?"
    print(f"\nQuery: {query}")
    print("\nTop 3 relevant rules:")

    results = rag.query(query, top_k=3)
    for i, (chunk, score) in enumerate(results, 1):
        print(f"\n{i}. [{chunk.source} {chunk.sutra_number or 'N/A'}] (relevance: {score:.3f})")
        print(f"   {chunk.text[:200]}...")

    # Example 2: Generate code implementation
    print("\n" + "=" * 60)
    print("Example 2: Generate Code from Rules")
    print("=" * 60)

    task = "Implement sandhi rule: a + i → e (vowel sandhi)"
    print(f"\nTask: {task}")
    print("\nGenerating Rust implementation...\n")

    code = suggest_implementation(
        rule_description=task, rag=rag, language="rust", include_tests=True
    )

    print("Generated code (⚠️ REVIEW REQUIRED):")
    print("-" * 60)
    print(code)
    print("-" * 60)
    print("\n⚠️  This is a STARTING POINT. You must:")
    print("   1. Review for correctness")
    print("   2. Add edge cases")
    print("   3. Test thoroughly")
    print("   4. Consult experts if uncertain")

    # Example 3: Generate test cases
    print("\n" + "=" * 60)
    print("Example 3: Generate Test Cases")
    print("=" * 60)

    print(f"\nGenerating test cases for: {task}")

    tests = generate_test_cases(function_description=task, rag=rag, num_cases=5)

    print(f"\nGenerated {len(tests)} test cases:\n")
    for i, test in enumerate(tests, 1):
        print(f"{i}. {test.get('description', 'Test case')}")
        print(f"   Input:    {test['input']}")
        print(f"   Expected: {test['expected']}")
        print()

    # Example 4: Explain a grammar concept
    print("=" * 60)
    print("Example 4: Explain Grammar Rule")
    print("=" * 60)

    print("\nExplaining: What is sandhi?\n")
    explanation = rag.explain_rule(query="What is sandhi in Sanskrit grammar?")
    print(explanation)

    print("\n" + "=" * 60)
    print("Summary")
    print("=" * 60)
    print("""
✅ RAG successfully retrieved relevant grammar rules
✅ LLM generated code implementation suggestions
✅ LLM generated test cases
✅ LLM provided natural language explanations

⚠️  REMEMBER: LLM assistance is a starting point, not the end!
   - Always verify against original sources
   - Test extensively with known examples
   - Consult experts for complex rules
   - Use LLM to accelerate, not replace, your work

Happy coding! 🙏
""")


if __name__ == "__main__":
    main()
