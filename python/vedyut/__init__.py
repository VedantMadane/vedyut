"""Vedyut - High-performance Sanskrit NLP Toolkit

A next-generation Sanskrit NLP toolkit combining Rust performance
with Python ease-of-use.

Script is a **first-class parameter** throughout the API.
"""

__version__ = "0.1.0"

from enum import Enum
from typing import List, Optional, Dict, Any


class Script(str, Enum):
    """
    Supported scripts for Sanskrit text.
    
    Script is a FIRST-CLASS parameter in vedyut, not buried in options.
    Every function that deals with script-specific text takes Script as
    an explicit, required parameter.
    """
    # Romanization schemes
    IAST = "iast"
    SLP1 = "slp1"
    HARVARD_KYOTO = "harvard-kyoto"
    ITRANS = "itrans"
    ISO15919 = "iso15919"
    VELTHUIS = "velthuis"
    WX = "wx"
    
    # Brahmic scripts
    DEVANAGARI = "devanagari"
    TELUGU = "telugu"
    TAMIL = "tamil"
    KANNADA = "kannada"
    MALAYALAM = "malayalam"
    BENGALI = "bengali"
    GUJARATI = "gujarati"
    GURMUKHI = "gurmukhi"
    ODIA = "odia"
    ASSAMESE = "assamese"
    TIBETAN = "tibetan"
    SINHALA = "sinhala"
    BURMESE = "burmese"
    THAI = "thai"
    GRANTHA = "grantha"


def transliterate(text: str, from_script: Script, to_script: Script) -> str:
    """
    Transliterate Sanskrit text between scripts.
    
    Script is a **first-class parameter** - explicit and required.
    
    Args:
        text: Text to transliterate
        from_script: Source script (first-class parameter!)
        to_script: Target script (first-class parameter!)
        
    Returns:
        Transliterated text
        
    Examples:
        >>> transliterate("namaste", Script.IAST, Script.DEVANAGARI)
        'नमस्ते'
        
        >>> transliterate("namaste", Script.IAST, Script.TAMIL)
        'நமஸ்தே'
        
        >>> transliterate("namaste", Script.IAST, Script.TELUGU)
        'నమస్తే'
    """
    # TODO: Call Rust core when built
    # from ._core import transliterate as _transliterate
    # return _transliterate(text, from_script.value, to_script.value)
    
    if from_script == to_script:
        return text
    return f"[TODO: Transliterate '{text}' from {from_script.value} to {to_script.value}]"


def segment(
    text: str,
    script: Script = Script.DEVANAGARI,
    max_results: int = 10,
) -> List[List[str]]:
    """
    Segment Sanskrit text into words.
    
    Script is explicitly specified (default: Devanagari).
    
    Args:
        text: Sanskrit text to segment
        script: Input script (first-class parameter with sensible default)
        max_results: Maximum number of segmentations to return
        
    Returns:
        List of possible segmentations, each as a list of words
        
    Examples:
        >>> segment("धर्मक्षेत्रे कुरुक्षेत्रे", Script.DEVANAGARI)
        [['धर्मक्षेत्रे', 'कुरुक्षेत्रे']]
        
        >>> segment("dharmakṣetre kurukṣetre", Script.IAST)
        [['dharmakṣetre', 'kurukṣetre']]
    """
    # TODO: Call Rust core when built
    # from ._core import segment as _segment
    # return _segment(text, script.value, max_results)
    
    # Placeholder
    return [text.split()]


def analyze(
    word: str,
    script: Script = Script.DEVANAGARI,
) -> List[Dict[str, Any]]:
    """
    Analyze morphological features of a Sanskrit word.
    
    Script is explicitly specified (default: Devanagari).
    
    Args:
        word: Sanskrit word to analyze
        script: Input script (first-class parameter)
        
    Returns:
        List of possible analyses with grammatical features
        
    Examples:
        >>> analyze("रामः", Script.DEVANAGARI)
        [{'stem': 'राम', 'case': 'nominative', 'number': 'singular', ...}]
    """
    # TODO: Call Rust core when built
    # from ._core import analyze as _analyze
    # return _analyze(word, script.value)
    
    # Placeholder
    return [{"word": word, "script": script.value}]


def generate_verb(
    dhatu: str,
    lakara: str,
    purusha: str,
    vacana: str,
    output_script: Script = Script.DEVANAGARI,
) -> List[str]:
    """
    Generate Sanskrit verb forms from root + grammatical features.
    
    Output script is explicitly specified (default: Devanagari).
    
    Args:
        dhatu: Verb root
        lakara: Tense/mood (lat, lit, lut, etc.)
        purusha: Person (prathama, madhyama, uttama)
        vacana: Number (eka, dvi, bahu)
        output_script: Output script (first-class parameter!)
        
    Returns:
        List of generated forms
        
    Examples:
        >>> generate_verb("भू", "lat", "prathama", "eka", Script.DEVANAGARI)
        ['भवति']
        
        >>> generate_verb("bhū", "lat", "prathama", "eka", Script.IAST)
        ['bhavati']
    """
    # TODO: Call Rust core when built
    # from ._core import generate_verb as _generate
    # return _generate(dhatu, lakara, purusha, vacana, output_script.value)
    
    # Placeholder
    return [f"{dhatu}+{lakara}+{purusha}+{vacana}"]


def list_scripts() -> List[Script]:
    """
    Get all supported scripts.
    
    Returns:
        List of all Script enum values
    """
    return list(Script)


def sanskritify(
    text: str,
    script: Script = Script.DEVANAGARI,
    level: str = "medium",
    preserve_meaning: bool = True,
    replace_urdu_arabic: bool = True,
    use_llm_fallback: bool = True,
    llm_api_key: Optional[str] = None,
) -> str:
    """
    Make text in any Indian language more like refined Sanskrit.
    
    Transforms modern colloquial text to use Sanskrit-style vocabulary,
    grammar patterns, and formal register. Works with ALL scripts!
    
    **NEW**: Automatically replaces Urdu/Arabic/Persian words with Sanskrit equivalents.
    Uses LLM fallback for words not in vocabulary database.
    
    Args:
        text: Text to sanskritify
        script: Script for input/output (first-class parameter!)
        level: Refinement level ("light", "medium", "high", "classical")
        preserve_meaning: Preserve original meaning vs. prioritize form
        replace_urdu_arabic: Replace Urdu/Arabic/Persian words with Sanskrit (default: True)
        use_llm_fallback: Use LLM for words not in vocabulary (default: True)
        llm_api_key: API key for LLM provider (OpenAI, Anthropic, etc.)
        
    Returns:
        Sanskritified text
        
    Examples:
        >>> # Basic sanskritification
        >>> sanskritify("hello friend", Script.DEVANAGARI)
        'नमस्ते मित्र'
        
        >>> # Works with any Indian script
        >>> sanskritify("hello friend", Script.TAMIL)
        'நமஸ்தே மித்ர'
        
        >>> # Replace Urdu/Arabic words automatically
        >>> sanskritify("duniya mein kitab", Script.DEVANAGARI)
        'जगत् में पुस्तक'
        
        >>> # High refinement with LLM fallback
        >>> sanskritify("salaam duniya", Script.DEVANAGARI, 
        ...             level="high", use_llm_fallback=True)
        'नमस्कार विश्व'
    """
    # TODO: Call Rust core when built
    # from ._core import sanskritify as _sanskritify
    # return _sanskritify(text, script.value, level, preserve_meaning)
    
    # Placeholder
    return f"[TODO: Sanskritify '{text}' in {script.value} at {level} level]"


__all__ = [
    "__version__",
    "Script",
    "transliterate",
    "segment",
    "analyze",
    "generate_verb",
    "sanskritify",
    "list_scripts",
]
