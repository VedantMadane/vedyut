"""Tests for FastAPI endpoints"""

import pytest
from fastapi.testclient import TestClient
from vedyut.api.main import app

client = TestClient(app)


def test_root():
    """Test root endpoint"""
    response = client.get("/")
    assert response.status_code == 200
    data = response.json()
    assert data["name"] == "Vedyut Sanskrit NLP API"
    assert "version" in data


def test_health():
    """Test health check endpoint"""
    response = client.get("/health")
    assert response.status_code == 200
    data = response.json()
    assert data["status"] == "ok"


def test_transliterate():
    """Test transliteration endpoint"""
    payload = {"text": "dharmakṣetre", "from_scheme": "iast", "to_scheme": "devanagari"}
    response = client.post("/v1/transliterate", json=payload)
    assert response.status_code == 200
    data = response.json()
    assert "result" in data
    assert "took_ms" in data
    assert data["from_scheme"] == "iast"
    assert data["to_scheme"] == "devanagari"


def test_segment():
    """Test segmentation endpoint"""
    payload = {"text": "धर्मक्षेत्रे कुरुक्षेत्रे", "max_splits": 10, "scheme": "devanagari"}
    response = client.post("/v1/segment", json=payload)
    assert response.status_code == 200
    data = response.json()
    assert "segments" in data
    assert "took_ms" in data
    assert isinstance(data["segments"], list)


def test_analyze():
    """Test morphological analysis endpoint"""
    payload = {"word": "रामः", "scheme": "devanagari"}
    response = client.post("/v1/analyze", json=payload)
    assert response.status_code == 200
    data = response.json()
    assert "word" in data
    assert "analyses" in data
    assert "took_ms" in data
    assert isinstance(data["analyses"], list)


def test_generate():
    """Test word generation endpoint"""
    payload = {"dhatu": "भू", "lakara": "lat", "purusha": "prathama", "vacana": "eka"}
    response = client.post("/v1/generate", json=payload)
    assert response.status_code == 200
    data = response.json()
    assert "forms" in data
    assert "dhatu" in data
    assert "took_ms" in data
    assert isinstance(data["forms"], list)


def test_metrics():
    """Test metrics endpoint"""
    response = client.get("/metrics")
    assert response.status_code == 200
    data = response.json()
    assert "requests_total" in data
    assert "avg_latency_ms" in data
