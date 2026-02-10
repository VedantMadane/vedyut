"""LLM integration for Sanskrit NLP with grammar treatise RAG"""

from .client import LLMClient
from .rag import GrammarRAG
from .tasks import (
    disambiguate_segmentation,
    explain_grammar,
    generate_test_cases,
    suggest_implementation,
    translate_sanskrit,
)

__all__ = [
    "LLMClient",
    "GrammarRAG",
    "disambiguate_segmentation",
    "translate_sanskrit",
    "explain_grammar",
    "suggest_implementation",
    "generate_test_cases",
]
