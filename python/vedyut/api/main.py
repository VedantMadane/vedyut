"""FastAPI application for Vedyut Sanskrit NLP API"""

from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel, Field
from typing import List, Optional
import time

app = FastAPI(
    title="Vedyut Sanskrit NLP API",
    description="High-performance Sanskrit NLP toolkit with Rust core",
    version="0.1.0",
    docs_url="/docs",
    redoc_url="/redoc",
)

# CORS middleware for web demos
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


# ===== Request/Response Models =====

class TransliterateRequest(BaseModel):
    """Request model for transliteration"""
    text: str = Field(..., description="Text to transliterate")
    from_scheme: str = Field(..., description="Source script (iast, slp1, devanagari, etc.)")
    to_scheme: str = Field(..., description="Target script (iast, slp1, devanagari, etc.)")


class TransliterateResponse(BaseModel):
    """Response model for transliteration"""
    result: str
    from_scheme: str
    to_scheme: str
    took_ms: float


class SegmentRequest(BaseModel):
    """Request model for segmentation"""
    text: str = Field(..., description="Sanskrit text to segment")
    max_splits: int = Field(10, description="Maximum number of segmentation options")
    scheme: str = Field("devanagari", description="Input script scheme")


class SegmentResponse(BaseModel):
    """Response model for segmentation"""
    segments: List[List[str]]
    took_ms: float


class AnalyzeRequest(BaseModel):
    """Request model for morphological analysis"""
    word: str = Field(..., description="Sanskrit word to analyze")
    scheme: str = Field("devanagari", description="Input script scheme")


class AnalysisResult(BaseModel):
    """Morphological analysis result"""
    lemma: str
    case: Optional[str] = None
    number: Optional[str] = None
    gender: Optional[str] = None
    person: Optional[str] = None
    tense: Optional[str] = None


class AnalyzeResponse(BaseModel):
    """Response model for analysis"""
    word: str
    analyses: List[AnalysisResult]
    took_ms: float


class GenerateRequest(BaseModel):
    """Request model for word generation"""
    dhatu: str = Field(..., description="Verb root (dhatu)")
    lakara: str = Field(..., description="Tense/mood (lakara)")
    purusha: str = Field(..., description="Person (prathama, madhyama, uttama)")
    vacana: str = Field(..., description="Number (eka, dvi, bahu)")


class GenerateResponse(BaseModel):
    """Response model for generation"""
    forms: List[str]
    dhatu: str
    took_ms: float


# ===== API Endpoints =====

@app.get("/")
async def root():
    """Root endpoint with API information"""
    return {
        "name": "Vedyut Sanskrit NLP API",
        "version": "0.1.0",
        "docs": "/docs",
        "health": "/health",
    }


@app.get("/health")
async def health():
    """Health check endpoint"""
    return {"status": "ok", "service": "vedyut"}


@app.post("/v1/transliterate", response_model=TransliterateResponse)
async def transliterate(req: TransliterateRequest):
    """
    Transliterate Sanskrit text between different scripts
    
    Supported schemes: devanagari, iast, slp1, hk (harvard-kyoto), itrans
    """
    start_time = time.time()
    
    try:
        # TODO: Call Rust core for actual transliteration
        result = f"[TODO: Transliterate '{req.text}' from {req.from_scheme} to {req.to_scheme}]"
        
        took_ms = (time.time() - start_time) * 1000
        
        return TransliterateResponse(
            result=result,
            from_scheme=req.from_scheme,
            to_scheme=req.to_scheme,
            took_ms=took_ms,
        )
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/v1/segment", response_model=SegmentResponse)
async def segment(req: SegmentRequest):
    """
    Segment Sanskrit text into words
    
    Returns multiple possible segmentations ranked by likelihood
    """
    start_time = time.time()
    
    try:
        # TODO: Call Rust core for actual segmentation
        # Placeholder: return mock segmentation
        segments = [
            req.text.split(),  # Simple space split as placeholder
        ]
        
        took_ms = (time.time() - start_time) * 1000
        
        return SegmentResponse(
            segments=segments,
            took_ms=took_ms,
        )
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/v1/analyze", response_model=AnalyzeResponse)
async def analyze(req: AnalyzeRequest):
    """
    Perform morphological analysis on a Sanskrit word
    
    Returns possible analyses with grammatical features
    """
    start_time = time.time()
    
    try:
        # TODO: Call Rust core for actual analysis
        # Placeholder: return mock analysis
        analyses = [
            AnalysisResult(
                lemma=req.word,
                case="nominative",
                number="singular",
            )
        ]
        
        took_ms = (time.time() - start_time) * 1000
        
        return AnalyzeResponse(
            word=req.word,
            analyses=analyses,
            took_ms=took_ms,
        )
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/v1/generate", response_model=GenerateResponse)
async def generate(req: GenerateRequest):
    """
    Generate Sanskrit word forms from root + grammatical features
    
    Generates tiṅanta (verb) forms following Pāṇinian grammar
    """
    start_time = time.time()
    
    try:
        # TODO: Call Rust core for actual generation
        # Placeholder: return mock form
        forms = [f"{req.dhatu}+{req.lakara}+{req.purusha}+{req.vacana}"]
        
        took_ms = (time.time() - start_time) * 1000
        
        return GenerateResponse(
            forms=forms,
            dhatu=req.dhatu,
            took_ms=took_ms,
        )
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


class SanskritifyRequest(BaseModel):
    """Request model for sanskritification"""
    text: str = Field(..., description="Text to sanskritify (any Indian language)")
    script: str = Field("devanagari", description="Script for input/output")
    level: str = Field("medium", description="Refinement level: light, medium, high, classical")
    preserve_meaning: bool = Field(True, description="Preserve original meaning")


class SanskritifyResponse(BaseModel):
    """Response model for sanskritification"""
    original: str
    refined: str
    script: str
    level: str
    took_ms: float


@app.post("/v1/sanskritify", response_model=SanskritifyResponse)
async def sanskritify_text(req: SanskritifyRequest):
    """
    Make text in any Indian language more like refined Sanskrit
    
    Transforms modern colloquial text to use Sanskrit-style vocabulary,
    grammar patterns, and formal register.
    
    Supports ALL Indian scripts: Devanagari, Tamil, Telugu, Malayalam,
    Kannada, Bengali, Gujarati, Gurmukhi, etc.
    """
    start_time = time.time()
    
    try:
        # TODO: Call Rust core for actual sanskritification
        # Placeholder transformation
        refined = f"[Sanskritified: {req.text}]"
        
        took_ms = (time.time() - start_time) * 1000
        
        return SanskritifyResponse(
            original=req.text,
            refined=refined,
            script=req.script,
            level=req.level,
            took_ms=took_ms,
        )
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/metrics")
async def metrics():
    """Basic API metrics (placeholder)"""
    return {
        "requests_total": 0,
        "avg_latency_ms": 0,
        "uptime_seconds": 0,
    }


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
