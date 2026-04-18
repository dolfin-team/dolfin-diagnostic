# dolfin-diagnostic

Unified diagnostic types for the Dolfin toolchain.

This is a **leaf crate** (i.e. it has no workspace dependencies) so every other crate in the toolchain can depend on it without creating cycles.

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `lsp`   | off     | LSP position/span conversion utilities (requires `lsp-types`) |

```toml
# Cargo.toml

# core types only
dolfin-diagnostic = { version = "..." }

# with LSP support
dolfin-diagnostic = { version = "...", features = ["lsp"] }
```

## Core types

### `Severity`

Four levels, ordered from lowest to highest:

```
Hint < Info < Warning < Error
```

### `DiagnosticCode`

A namespaced code identifying the kind of diagnostic:

| Variant | Format | Example | Origin |
|---------|--------|---------|--------|
| `Parse(u16)` | `P<n>` | `P200` | rowl lexer / parser errors |
| `Semantic(u16)` | `S<nnn>` | `S001` | dolfin-analysis |
| `Lint(String)` | `L/<id>` | `L/naming/pascal-case` | dolfin-lint |

Built-in semantic constants:

```rust
DiagnosticCode::UNRESOLVED_TYPE       // S001
DiagnosticCode::DUPLICATE_DECLARATION // S002
DiagnosticCode::CIRCULAR_INHERITANCE  // S003
```

### `Span` and `Location`

`Location` is a 1-based line/column pair plus a 0-based byte offset. `Span` is an inclusive start / exclusive end pair of `Location`s.

```rust
let loc = Location::new(line, column, offset);
let span = Span::new(start, end);
let point = Span::at(loc);          // zero-width insertion point
let merged = span_a.merge(&span_b); // smallest span covering both
```

## Building diagnostics

Use `DiagnosticBuilder` for a fluent API:

```rust
use dolfin_diagnostic::{DiagnosticBuilder, DiagnosticCode};

let diag = DiagnosticBuilder::error(DiagnosticCode::Parse(200), "unexpected token")
    .span(span)
    .label(other_span, "hint: token started here")
    .help("add a colon after the concept name")
    .build();
```

Shorthand constructors: `DiagnosticBuilder::error`, `::warning`, `::info`, `::hint`.

## Fix suggestions

A `FixSuggestion` carries a human-readable description and a list of `TextEdit`s that tools can apply automatically.

```rust
use dolfin_diagnostic::{FixSuggestion, TextEdit};

// Single-edit fix in the same file
let fix = FixSuggestion::single("insert semicolon", span, ";");

// Multi-edit fix, potentially across files
let fix = FixSuggestion::new("rename symbol", vec![
    TextEdit::new(span_a, "new_name"),
    TextEdit::new_in_file("/other/file.dol", span_b, "new_name"),
]);
```

## Collecting diagnostics

`DiagnosticAccumulator` collects diagnostics during a compilation pass:

```rust
use dolfin_diagnostic::DiagnosticAccumulator;

let mut acc = DiagnosticAccumulator::new();
acc.push(diag);
acc.extend(more_diags);

if acc.has_errors() {
    return Err(acc.into_diagnostics());
}
```

## Rendering

```rust
use dolfin_diagnostic::{format_diagnostic, print_diagnostics, print_summary};
use std::path::Path;

// Format one diagnostic to a String (e.g. for tests or logging)
let text = format_diagnostic(&diag, Some(source), Some("main.dol"));

// Print all diagnostics to stderr with source context
print_diagnostics(&diags, source, Path::new("main.dol"));

// Print "N error(s), M warning(s)" summary to stderr
print_summary(&diags);
```

Example terminal output:

```
error[P200]: unexpected token
  --> main.dlf:3:13
   |
 3 | concept Foo Bar
   |             ^^^
  = help: add a colon after the concept name
```

## LSP integration (`lsp` feature)

```rust
use dolfin_diagnostic::lsp::{location_to_position, span_to_range, span_contains};

let position = location_to_position(loc);   // Location  → lsp_types::Position
let range    = span_to_range(span);         // Span      → lsp_types::Range
let location = position_to_location(pos);   // lsp_types::Position → Location

let hit = span_contains(diag.span, byte_offset);
```

`Severity` also implements `From<Severity> for lsp_types::DiagnosticSeverity` under the `lsp` feature.

## License

See [LICENSE](LICENSE).
