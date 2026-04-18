use crate::{Diagnostic, Severity};

/// Collects diagnostics during a pass (parsing, analysis, linting).
#[derive(Debug, Default)]
pub struct DiagnosticAccumulator {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticAccumulator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, d: Diagnostic) {
        self.diagnostics.push(d);
    }

    pub fn extend(&mut self, iter: impl IntoIterator<Item = Diagnostic>) {
        self.diagnostics.extend(iter);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == Severity::Error)
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}
