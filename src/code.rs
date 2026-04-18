use std::fmt;

/// Unified diagnostic code namespace for the entire Dolfin toolchain.
///
/// Code prefixes:
/// - `P1xx` — lexer errors (was `E1xx` in rowl)
/// - `P2xx` — parser errors (was `E2xx` in rowl)
/// - `P3xx` — rowl semantic errors (was `E3xx` in rowl)
/// - `S001`… — semantic analysis codes (was `E001`-`E003` in dolfin-analysis)
/// - `L/<rule-id>` — lint rule violations (was free-form strings in dolfin-lint)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiagnosticCode {
    /// Parse / lexer error identified by its numeric code (P prefix).
    /// Use the numeric value from `rowl::error::ErrorCode as u16`.
    Parse(u16),
    /// Semantic analysis code (S prefix, zero-padded to 3 digits).
    Semantic(u16),
    /// Lint rule violation, identified by the rule's string ID.
    Lint(String),
}

impl DiagnosticCode {
    /// Canonical string representation of the code.
    ///
    /// Examples: `"P200"`, `"S001"`, `"L/naming/pascal-case"`
    pub fn code_str(&self) -> String {
        match self {
            DiagnosticCode::Parse(n) => format!("P{n}"),
            DiagnosticCode::Semantic(n) => format!("S{n:03}"),
            DiagnosticCode::Lint(id) => format!("L/{id}"),
        }
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.code_str())
    }
}

// ── Semantic code constants ───────────────────────────────────────────────────

impl DiagnosticCode {
    pub const UNRESOLVED_TYPE: DiagnosticCode = DiagnosticCode::Semantic(1);
    pub const DUPLICATE_DECLARATION: DiagnosticCode = DiagnosticCode::Semantic(2);
    pub const CIRCULAR_INHERITANCE: DiagnosticCode = DiagnosticCode::Semantic(3);
}
