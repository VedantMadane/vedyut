"""Unified LLM client with swappable backends via LiteLLM"""

import os
from typing import List, Dict, Optional, Any
import litellm
from litellm import completion, embedding

# Suppress LiteLLM verbose logging
litellm.suppress_debug_info = True


class LLMClient:
    """Unified LLM client supporting 100+ providers via LiteLLM

    Supported models:
    - OpenAI: gpt-4o, gpt-4-turbo, gpt-3.5-turbo
    - Anthropic: claude-3-5-sonnet-20241022, claude-3-opus
    - Google: gemini/gemini-1.5-pro, gemini/gemini-1.5-flash
    - Azure, AWS Bedrock, Ollama, etc.

    Configuration via environment variables:
    - VEDYUT_LLM_MODEL: Model name (default: gpt-4o)
    - OPENAI_API_KEY, ANTHROPIC_API_KEY, GOOGLE_API_KEY, etc.
    """

    DEFAULT_MODEL = "gpt-4o"
    DEFAULT_EMBEDDING_MODEL = "text-embedding-3-large"

    def __init__(
        self,
        model: Optional[str] = None,
        embedding_model: Optional[str] = None,
        temperature: float = 0.7,
        max_tokens: Optional[int] = None,
        api_key: Optional[str] = None,
    ):
        """Initialize LLM client

        Args:
            model: Model name (e.g., "gpt-4o", "claude-3-5-sonnet-20241022")
            embedding_model: Model for embeddings
            temperature: Sampling temperature (0.0-1.0)
            max_tokens: Max tokens in response
            api_key: Optional API key (or use env vars)
        """
        self.model = model or os.getenv("VEDYUT_LLM_MODEL", self.DEFAULT_MODEL)
        self.embedding_model = embedding_model or os.getenv(
            "VEDYUT_EMBEDDING_MODEL", self.DEFAULT_EMBEDDING_MODEL
        )
        self.temperature = temperature
        self.max_tokens = max_tokens

        # LiteLLM auto-detects API keys from env (OPENAI_API_KEY, etc.)
        if api_key:
            litellm.api_key = api_key

    def complete(self, messages: List[Dict[str, str]], **kwargs) -> str:
        """Complete a chat conversation

        Args:
            messages: List of {"role": "user/assistant/system", "content": "..."}
            **kwargs: Additional args passed to LiteLLM (temperature, max_tokens, etc.)

        Returns:
            Response text
        """
        response = completion(
            model=self.model,
            messages=messages,
            temperature=kwargs.get("temperature", self.temperature),
            max_tokens=kwargs.get("max_tokens", self.max_tokens),
            **{k: v for k, v in kwargs.items() if k not in ["temperature", "max_tokens"]},
        )
        return response.choices[0].message.content

    def complete_with_json(self, messages: List[Dict[str, str]], **kwargs) -> Dict[str, Any]:
        """Complete with structured JSON response

        Args:
            messages: Chat messages
            **kwargs: Additional args

        Returns:
            Parsed JSON response as dict
        """
        response = completion(
            model=self.model,
            messages=messages,
            response_format={"type": "json_object"},
            temperature=kwargs.get("temperature", self.temperature),
            max_tokens=kwargs.get("max_tokens", self.max_tokens),
            **{
                k: v
                for k, v in kwargs.items()
                if k not in ["temperature", "max_tokens", "response_format"]
            },
        )

        import json

        content = response.choices[0].message.content
        return json.loads(content)

    def embed(self, texts: List[str]) -> List[List[float]]:
        """Generate embeddings for texts

        Args:
            texts: List of text strings to embed

        Returns:
            List of embedding vectors
        """
        if isinstance(texts, str):
            texts = [texts]

        response = embedding(model=self.embedding_model, input=texts)
        return [item["embedding"] for item in response.data]

    def embed_single(self, text: str) -> List[float]:
        """Generate embedding for a single text

        Args:
            text: Text to embed

        Returns:
            Embedding vector
        """
        return self.embed([text])[0]

    def stream(self, messages: List[Dict[str, str]], **kwargs):
        """Stream completion response (for long responses)

        Args:
            messages: Chat messages
            **kwargs: Additional args

        Yields:
            Response chunks
        """
        response = completion(
            model=self.model,
            messages=messages,
            stream=True,
            temperature=kwargs.get("temperature", self.temperature),
            max_tokens=kwargs.get("max_tokens", self.max_tokens),
            **{k: v for k, v in kwargs.items() if k not in ["temperature", "max_tokens", "stream"]},
        )

        for chunk in response:
            if chunk.choices[0].delta.content:
                yield chunk.choices[0].delta.content


# Convenience function for quick use
def quick_complete(prompt: str, model: Optional[str] = None) -> str:
    """Quick one-off completion (not for production)

    Args:
        prompt: User prompt
        model: Optional model override

    Returns:
        Response text
    """
    client = LLMClient(model=model)
    return client.complete([{"role": "user", "content": prompt}])
