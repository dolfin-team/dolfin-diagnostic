use std::fmt;

/// Canonical severity level used across the entire Dolfin toolchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    /// Informational hint.
    Hint = 0,
    /// Informational message (e.g. style suggestions).
    Info = 1,
    /// Warning – code is valid but may be problematic.
    Warning = 2,
    /// Error – code is invalid.
    Error = 3,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Severity::Hint => "hint",
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
        }
    }

    pub fn is_error(self) -> bool {
        self == Severity::Error
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "lsp")]
impl From<Severity> for lsp_types::DiagnosticSeverity {
    fn from(s: Severity) -> Self {
        match s {
            Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
            Severity::Warning => lsp_types::DiagnosticSeverity::WARNING,
            Severity::Info => lsp_types::DiagnosticSeverity::INFORMATION,
            Severity::Hint => lsp_types::DiagnosticSeverity::HINT,
        }
    }
}
