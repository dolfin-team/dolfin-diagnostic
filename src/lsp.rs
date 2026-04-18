//! LSP position/span conversion utilities.
//!
//! Enabled with the `lsp` feature flag.

use lsp_types;

use crate::{Location, Span};

/// rowl `Location` (1-based line/col) → LSP `Position` (0-based).
pub fn location_to_position(loc: Location) -> lsp_types::Position {
    lsp_types::Position {
        line: loc.line.saturating_sub(1) as u32,
        character: loc.column.saturating_sub(1) as u32,
    }
}

/// rowl `Span` → LSP `Range`.
pub fn span_to_range(span: Span) -> lsp_types::Range {
    lsp_types::Range {
        start: location_to_position(span.start),
        end: location_to_position(span.end),
    }
}

/// LSP `Position` (0-based) → rowl `Location` (byte offset = 0; fill with `offset_of`).
pub fn position_to_location(pos: lsp_types::Position) -> Location {
    Location {
        line: pos.line as usize + 1,
        column: pos.character as usize + 1,
        offset: 0,
    }
}

/// Returns `true` if `span` contains `byte_offset`.
pub fn span_contains(span: Option<Span>, byte_offset: usize) -> bool {
    span.map_or(false, |s| s.start.offset <= byte_offset && byte_offset <= s.end.offset)
}
