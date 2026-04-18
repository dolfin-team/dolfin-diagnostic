use crate::Span;

/// A suggested code fix attached to a diagnostic.
#[derive(Debug, Clone)]
pub struct FixSuggestion {
    pub description: String,
    pub edits: Vec<TextEdit>,
}

impl FixSuggestion {
    pub fn new(description: impl Into<String>, edits: Vec<TextEdit>) -> Self {
        Self { description: description.into(), edits }
    }

    /// Convenience: single-edit fix replacing `span` with `replacement`.
    pub fn single(description: impl Into<String>, span: Span, replacement: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            edits: vec![TextEdit { file: String::new(), span, replacement: replacement.into() }],
        }
    }
}

/// A single text replacement within a source file.
#[derive(Debug, Clone)]
pub struct TextEdit {
    /// Absolute path of the file to edit. Empty string means "same file as the diagnostic".
    pub file: String,
    pub span: Span,
    pub replacement: String,
}

impl TextEdit {
    pub fn new(span: Span, replacement: impl Into<String>) -> Self {
        Self { file: String::new(), span, replacement: replacement.into() }
    }

    pub fn new_in_file(file: impl Into<String>, span: Span, replacement: impl Into<String>) -> Self {
        Self { file: file.into(), span, replacement: replacement.into() }
    }
}
