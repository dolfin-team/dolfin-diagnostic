use crate::{DiagnosticCode, FixSuggestion, Severity, Span};

/// A single diagnostic message — error, warning, info, or hint —
/// with full source context.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: DiagnosticCode,
    pub message: String,
    /// Primary span in the source file. `None` means the whole file.
    pub span: Option<Span>,
    /// Secondary labeled spans providing additional context.
    pub labels: Vec<DiagnosticLabel>,
    /// Optional help text suggesting how to fix the issue.
    pub help: Option<String>,
    /// Optional automated fix suggestion.
    pub fix: Option<FixSuggestion>,
}

impl Diagnostic {
    pub fn is_error(&self) -> bool {
        self.severity.is_error()
    }
}

/// A labeled secondary span within a diagnostic.
#[derive(Debug, Clone)]
pub struct DiagnosticLabel {
    pub span: Span,
    pub message: String,
}

impl DiagnosticLabel {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self { span, message: message.into() }
    }
}

// ── Builder ───────────────────────────────────────────────────────────────────

/// Fluent builder for constructing [`Diagnostic`]s.
///
/// # Example
/// ```ignore
/// DiagnosticBuilder::error(DiagnosticCode::Parse(200), "unexpected token")
///     .span(span)
///     .help("add a colon after the concept name")
///     .build()
/// ```
pub struct DiagnosticBuilder {
    severity: Severity,
    code: DiagnosticCode,
    message: String,
    span: Option<Span>,
    help: Option<String>,
    labels: Vec<DiagnosticLabel>,
    fix: Option<FixSuggestion>,
}

impl DiagnosticBuilder {
    pub fn new(severity: Severity, code: DiagnosticCode, message: impl Into<String>) -> Self {
        Self {
            severity,
            code,
            message: message.into(),
            span: None,
            help: None,
            labels: vec![],
            fix: None,
        }
    }

    pub fn error(code: DiagnosticCode, message: impl Into<String>) -> Self {
        Self::new(Severity::Error, code, message)
    }

    pub fn warning(code: DiagnosticCode, message: impl Into<String>) -> Self {
        Self::new(Severity::Warning, code, message)
    }

    pub fn info(code: DiagnosticCode, message: impl Into<String>) -> Self {
        Self::new(Severity::Info, code, message)
    }

    pub fn hint(code: DiagnosticCode, message: impl Into<String>) -> Self {
        Self::new(Severity::Hint, code, message)
    }

    pub fn span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn span_opt(mut self, span: Option<Span>) -> Self {
        self.span = span;
        self
    }

    pub fn at(mut self, location: crate::Location) -> Self {
        self.span = Some(Span::at(location));
        self
    }

    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn label(mut self, span: Span, message: impl Into<String>) -> Self {
        self.labels.push(DiagnosticLabel::new(span, message));
        self
    }

    pub fn fix(mut self, fix: FixSuggestion) -> Self {
        self.fix = Some(fix);
        self
    }

    pub fn build(self) -> Diagnostic {
        Diagnostic {
            severity: self.severity,
            code: self.code,
            message: self.message,
            span: self.span,
            help: self.help,
            labels: self.labels,
            fix: self.fix,
        }
    }
}
