//! Vedyut Core - Main library with Python bindings
//!
//! This crate combines all vedyut modules and provides Python bindings via PyO3.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use vedyut_lipi::Scheme;

/// Python module for vedyut
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register classes and functions
    m.add_class::<PyScheme>()?;
    m.add_function(wrap_pyfunction!(py_transliterate, m)?)?;
    m.add_function(wrap_pyfunction!(py_sanskritify, m)?)?;
    m.add_function(wrap_pyfunction!(py_segment, m)?)?;
    m.add_function(wrap_pyfunction!(py_analyze, m)?)?;

    Ok(())
}

/// Python wrapper for Scheme enum
#[pyclass(name = "Scheme")]
#[derive(Clone)]
struct PyScheme {
    inner: Scheme,
}

#[pymethods]
impl PyScheme {
    #[new]
    fn new(name: &str) -> PyResult<Self> {
        let scheme = Scheme::from_str(name).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Unsupported scheme: {}",
                name
            ))
        })?;
        Ok(Self { inner: scheme })
    }

    fn __repr__(&self) -> String {
        format!("Scheme('{}')", self.inner.name())
    }

    fn __str__(&self) -> String {
        self.inner.name().to_string()
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.name().to_string()
    }
}

/// Transliterate text between scripts
#[pyfunction]
#[pyo3(signature = (text, from_scheme, to_scheme))]
fn py_transliterate(text: &str, from_scheme: &str, to_scheme: &str) -> PyResult<String> {
    let from = Scheme::from_str(from_scheme).ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Unsupported source scheme: {}",
            from_scheme
        ))
    })?;

    let to = Scheme::from_str(to_scheme).ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Unsupported target scheme: {}",
            to_scheme
        ))
    })?;

    let result = vedyut_lipi::transliterate(text, from, to);
    Ok(result)
}

/// Sanskritify text
#[pyfunction]
#[pyo3(signature = (text, script, level="medium", preserve_meaning=true, replace_urdu_arabic=true))]
fn py_sanskritify(
    text: &str,
    script: &str,
    level: &str,
    preserve_meaning: bool,
    replace_urdu_arabic: bool,
) -> PyResult<String> {
    use vedyut_sanskritify::{RefinementLevel, SanskritifyOptions};

    let scheme = Scheme::from_str(script).ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Unsupported script: {}", script))
    })?;

    let refinement_level = match level {
        "light" => RefinementLevel::Light,
        "medium" => RefinementLevel::Medium,
        "high" => RefinementLevel::High,
        "classical" => RefinementLevel::Classical,
        _ => RefinementLevel::Medium,
    };

    let options = SanskritifyOptions {
        level: refinement_level,
        preserve_meaning,
        replace_foreign_words: replace_urdu_arabic,
        ..Default::default()
    };

    vedyut_sanskritify::sanskritify_text(text, scheme, options)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

/// Segment text into words
#[pyfunction]
#[pyo3(signature = (text, script="devanagari", max_results=10))]
fn py_segment(text: &str, script: &str, max_results: usize) -> PyResult<Vec<Vec<String>>> {
    let _scheme = Scheme::from_str(script).ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Unsupported script: {}", script))
    })?;

    let results = vedyut_cheda::segment_text(text);

    Ok(results
        .into_iter()
        .take(max_results)
        .map(|r| r.words)
        .collect())
}

/// Analyze morphological features
#[pyfunction]
#[pyo3(signature = (word, script="devanagari"))]
fn py_analyze(word: &str, script: &str, py: Python) -> PyResult<Vec<PyObject>> {
    let _scheme = Scheme::from_str(script).ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Unsupported script: {}", script))
    })?;

    if let Some(analysis) = vedyut_cheda::analyze_word(word) {
        let dict = PyDict::new_bound(py);
        dict.set_item("word", analysis.word)?;
        dict.set_item("stem", analysis.stem)?;
        dict.set_item("linga", analysis.linga)?;
        dict.set_item("vibhakti", analysis.vibhakti)?;
        dict.set_item("vacana", analysis.vacana)?;
        dict.set_item("tags", analysis.tags)?;

        Ok(vec![dict.into()])
    } else {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let module = PyModule::new_bound(py, "_core").unwrap();
            assert!(_core(&module).is_ok());
        });
    }
}
