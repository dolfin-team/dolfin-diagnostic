//! Unified diagnostic infrastructure for the Dolfin toolchain.
//!
//! This is a leaf crate — it has no workspace dependencies — so every other
//! crate can depend on it without creating cycles.
//!
//! # Feature flags
//! - `lsp` — enables LSP position/span helpers (requires `lsp-types`).

mod accumulator;
mod code;
mod diagnostic;
mod fix;
mod render;
mod severity;
mod span;

#[cfg(feature = "lsp")]
pub mod lsp;

pub use accumulator::DiagnosticAccumulator;
pub use code::DiagnosticCode;
pub use diagnostic::{Diagnostic, DiagnosticBuilder, DiagnosticLabel};
pub use fix::{FixSuggestion, TextEdit};
pub use render::{format_diagnostic, print_diagnostics, print_summary};
pub use severity::Severity;
pub use span::{Location, Span};
