"""RAG (Retrieval-Augmented Generation) for Sanskrit grammar treatises

This module enables LLMs to reference Pāṇinian grammar texts:
- Aṣṭādhyāyī sūtras (Sanskrit)
- Kāśikā commentary (Sanskrit)
- English textbooks (Kale, Whitney, etc.)
- Modern explanations

The LLM can then:
1. Retrieve relevant sūtras for a grammar question
2. Generate code based on grammar rules
3. Explain rules in natural language
4. Cross-reference multiple sources
"""

import os
import json
from pathlib import Path
from typing import List, Dict, Optional, Tuple
import numpy as np
from dataclasses import dataclass, asdict

from .client import LLMClient


@dataclass
class GrammarChunk:
    """A chunk of grammar text with metadata"""

    id: str
    text: str  # The actual content (sūtra + commentary)
    source: str  # "ashtadhyayi", "kashika", "kale", etc.
    sutra_number: Optional[str] = None  # e.g., "1.1.1", "3.2.123"
    topic: Optional[str] = None  # e.g., "sandhi", "lakara", "dhatu"
    language: str = "sanskrit"  # "sanskrit" or "english"
    embedding: Optional[List[float]] = None


class GrammarRAG:
    """RAG system for Sanskrit grammar treatises

    Usage:
        rag = GrammarRAG(data_dir="data/grammar")
        rag.load_texts()  # Load grammar treatises
        rag.build_index()  # Generate embeddings

        # Query for relevant rules
        results = rag.query("How to form present tense verbs?", top_k=3)

        # Use with LLM
        code = rag.generate_code("Implement sandhi rule for 'a + i → e'")
    """

    def __init__(
        self,
        data_dir: str = "data/grammar",
        llm_client: Optional[LLMClient] = None,
        index_file: str = "grammar_index.json",
    ):
        """Initialize RAG system

        Args:
            data_dir: Directory containing grammar text files
            llm_client: LLM client for embeddings and generation
            index_file: File to save/load embedded chunks
        """
        self.data_dir = Path(data_dir)
        self.llm = llm_client or LLMClient()
        self.index_file = self.data_dir / index_file

        self.chunks: List[GrammarChunk] = []
        self.chunk_embeddings: Optional[np.ndarray] = None

    def load_texts(self):
        """Load grammar treatises from data directory

        Expected structure:
            data/grammar/
                ashtadhyayi.txt       # Sūtras in Sanskrit/SLP1
                kashika.txt           # Commentary in Sanskrit
                kale_grammar.txt      # English textbook
                panini_intro.txt      # Modern English explanations
                custom_rules.json     # Custom rule definitions
        """
        if not self.data_dir.exists():
            print(f"Warning: Grammar data directory not found: {self.data_dir}")
            print("Create it and add grammar texts to enable RAG functionality.")
            return

        # Load text files
        for file_path in self.data_dir.glob("*.txt"):
            self._load_text_file(file_path)

        # Load structured JSON files
        for file_path in self.data_dir.glob("*.json"):
            self._load_json_file(file_path)

        print(f"Loaded {len(self.chunks)} grammar chunks from {self.data_dir}")

    def _load_text_file(self, file_path: Path):
        """Load and chunk a text file"""
        source = file_path.stem  # e.g., "ashtadhyayi", "kale_grammar"
        language = "sanskrit" if any(x in source for x in ["ashtadhyayi", "kashika"]) else "english"

        with open(file_path, encoding="utf-8") as f:
            content = f.read()

        # Simple chunking by paragraphs (TODO: improve with sutra-aware chunking)
        paragraphs = [p.strip() for p in content.split("\n\n") if p.strip()]

        for i, para in enumerate(paragraphs):
            chunk = GrammarChunk(
                id=f"{source}_{i}",
                text=para,
                source=source,
                language=language,
                sutra_number=self._extract_sutra_number(para),
                topic=self._infer_topic(para),
            )
            self.chunks.append(chunk)

    def _load_json_file(self, file_path: Path):
        """Load structured grammar rules from JSON

        Expected format:
        [
            {
                "sutra": "1.1.1",
                "sanskrit": "वृद्धिरादैच्",
                "transliteration": "vṛddhir ādaic",
                "english": "a, ai, au are called vṛddhi",
                "explanation": "This defines the vṛddhi vowels...",
                "topic": "sandhi"
            },
            ...
        ]
        """
        with open(file_path, encoding="utf-8") as f:
            data = json.load(f)

        for i, rule in enumerate(data):
            # Create chunks for Sanskrit and English versions
            if "sanskrit" in rule:
                chunk = GrammarChunk(
                    id=f"{file_path.stem}_{i}_sa",
                    text=f"{rule.get('sutra', '')}: {rule['sanskrit']}\n{rule.get('explanation', '')}",
                    source=file_path.stem,
                    sutra_number=rule.get("sutra"),
                    topic=rule.get("topic"),
                    language="sanskrit",
                )
                self.chunks.append(chunk)

            if "english" in rule:
                chunk = GrammarChunk(
                    id=f"{file_path.stem}_{i}_en",
                    text=f"{rule.get('sutra', '')}: {rule['english']}\n{rule.get('explanation', '')}",
                    source=file_path.stem,
                    sutra_number=rule.get("sutra"),
                    topic=rule.get("topic"),
                    language="english",
                )
                self.chunks.append(chunk)

    def _extract_sutra_number(self, text: str) -> Optional[str]:
        """Extract sūtra number from text (e.g., '1.1.1', '3.2.123')"""
        import re

        match = re.search(r"\b(\d+\.\d+\.\d+)\b", text[:100])
        return match.group(1) if match else None

    def _infer_topic(self, text: str) -> Optional[str]:
        """Infer grammatical topic from text content"""
        text_lower = text.lower()
        if any(word in text_lower for word in ["sandhi", "सन्धि"]):
            return "sandhi"
        elif any(word in text_lower for word in ["lakara", "लकार", "tense", "वृत्ति"]):
            return "lakara"
        elif any(word in text_lower for word in ["dhatu", "धातु", "verb", "root"]):
            return "dhatu"
        elif any(word in text_lower for word in ["vibhakti", "विभक्ति", "case"]):
            return "vibhakti"
        elif any(word in text_lower for word in ["samasa", "समास", "compound"]):
            return "samasa"
        return None

    def build_index(self, force_rebuild: bool = False):
        """Generate embeddings for all chunks and build search index

        Args:
            force_rebuild: If True, rebuild even if index exists
        """
        # Try to load existing index
        if not force_rebuild and self.index_file.exists():
            self._load_index()
            print(f"Loaded existing index from {self.index_file}")
            return

        if not self.chunks:
            print("No chunks to index. Run load_texts() first.")
            return

        print(f"Generating embeddings for {len(self.chunks)} chunks...")
        texts = [chunk.text for chunk in self.chunks]

        # Generate embeddings in batches (API rate limits)
        batch_size = 100
        all_embeddings = []

        for i in range(0, len(texts), batch_size):
            batch = texts[i : i + batch_size]
            embeddings = self.llm.embed(batch)
            all_embeddings.extend(embeddings)
            print(f"  Embedded {min(i + batch_size, len(texts))}/{len(texts)}")

        # Store embeddings in chunks
        for chunk, embedding in zip(self.chunks, all_embeddings):
            chunk.embedding = embedding

        self.chunk_embeddings = np.array(all_embeddings)

        # Save index
        self._save_index()
        print(f"Index saved to {self.index_file}")

    def _save_index(self):
        """Save chunks and embeddings to disk"""
        self.data_dir.mkdir(parents=True, exist_ok=True)

        data = {"chunks": [asdict(chunk) for chunk in self.chunks], "version": "1.0"}

        with open(self.index_file, "w", encoding="utf-8") as f:
            json.dump(data, f, ensure_ascii=False, indent=2)

    def _load_index(self):
        """Load chunks and embeddings from disk"""
        with open(self.index_file, encoding="utf-8") as f:
            data = json.load(f)

        self.chunks = [GrammarChunk(**chunk) for chunk in data["chunks"]]
        self.chunk_embeddings = np.array([chunk.embedding for chunk in self.chunks])

    def query(
        self,
        query_text: str,
        top_k: int = 5,
        topic: Optional[str] = None,
        language: Optional[str] = None,
    ) -> List[Tuple[GrammarChunk, float]]:
        """Retrieve most relevant grammar chunks for a query

        Args:
            query_text: Natural language query (e.g., "How to form past tense?")
            top_k: Number of results to return
            topic: Filter by topic ("sandhi", "lakara", etc.)
            language: Filter by language ("sanskrit" or "english")

        Returns:
            List of (chunk, similarity_score) tuples, sorted by relevance
        """
        if self.chunk_embeddings is None:
            raise ValueError("Index not built. Run build_index() first.")

        # Generate query embedding
        query_embedding = self.llm.embed_single(query_text)
        query_vec = np.array(query_embedding)

        # Compute cosine similarity
        similarities = np.dot(self.chunk_embeddings, query_vec) / (
            np.linalg.norm(self.chunk_embeddings, axis=1) * np.linalg.norm(query_vec)
        )

        # Filter by topic/language if specified
        filtered_indices = []
        for i, chunk in enumerate(self.chunks):
            if topic and chunk.topic != topic:
                continue
            if language and chunk.language != language:
                continue
            filtered_indices.append(i)

        # Get top-k
        if filtered_indices:
            filtered_sims = [(i, similarities[i]) for i in filtered_indices]
            top_indices = sorted(filtered_sims, key=lambda x: x[1], reverse=True)[:top_k]
        else:
            top_indices = [(i, similarities[i]) for i in np.argsort(similarities)[::-1][:top_k]]

        results = [(self.chunks[i], float(score)) for i, score in top_indices]
        return results

    def generate_code(
        self,
        task_description: str,
        context_chunks: Optional[List[GrammarChunk]] = None,
        language: str = "rust",
    ) -> str:
        """Generate code implementation based on grammar rules

        Args:
            task_description: What to implement (e.g., "sandhi rule for a + i")
            context_chunks: Relevant grammar chunks (auto-retrieved if None)
            language: Target programming language

        Returns:
            Generated code with comments
        """
        # Retrieve relevant chunks if not provided
        if context_chunks is None:
            results = self.query(task_description, top_k=3)
            context_chunks = [chunk for chunk, _ in results]

        # Build context from chunks
        context_text = "\n\n".join(
            [
                f"[{chunk.source} {chunk.sutra_number or ''}]\n{chunk.text}"
                for chunk in context_chunks
            ]
        )

        prompt = f"""You are a Sanskrit NLP expert. Based on these Pāṇinian grammar rules, generate {language} code to implement the requested functionality.

Grammar References:
{context_text}

Task: {task_description}

Generate clean, well-commented {language} code. Include:
1. Function signature with types
2. Implementation logic
3. Comments explaining the grammar rule
4. Example usage in comments

{language.upper()} CODE:
"""

        messages = [{"role": "user", "content": prompt}]
        return self.llm.complete(messages, temperature=0.3)

    def explain_rule(
        self,
        sutra_number: Optional[str] = None,
        query: Optional[str] = None,
    ) -> str:
        """Get natural language explanation of a grammar rule

        Args:
            sutra_number: Specific sūtra (e.g., "1.1.1")
            query: Natural language query (if sutra_number not provided)

        Returns:
            Plain English explanation
        """
        if sutra_number:
            # Find chunks with this sutra
            matching_chunks = [c for c in self.chunks if c.sutra_number == sutra_number]
            if not matching_chunks:
                return f"Sūtra {sutra_number} not found in loaded texts."
            context_chunks = matching_chunks[:3]
        elif query:
            results = self.query(query, top_k=3)
            context_chunks = [chunk for chunk, _ in results]
        else:
            raise ValueError("Provide either sutra_number or query")

        context_text = "\n\n".join([chunk.text for chunk in context_chunks])

        prompt = f"""Explain this Pāṇinian grammar rule in simple, clear English.

Grammar Text:
{context_text}

Provide:
1. What the rule says
2. When it applies
3. A simple example
4. Common mistakes

EXPLANATION:
"""

        messages = [{"role": "user", "content": prompt}]
        return self.llm.complete(messages, temperature=0.5)
