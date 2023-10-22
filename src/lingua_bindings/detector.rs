use pyo3::prelude::*;
use lingua::LanguageDetector;

use super::language::PyLanguage;

#[pyclass]
pub struct PyLanguageDetector {
    pub(crate) detector: LanguageDetector,
}
#[pymethods]
impl PyLanguageDetector {
    pub fn detect_language_of(&self, text: String) -> Option<PyLanguage> {
        if let Some(lang) = self.detector.detect_language_of(text) {
            Some(lang.into())
        } else {
            None
        }
    }
}