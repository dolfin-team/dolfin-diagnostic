use std::path::Path;

use crate::{Diagnostic, Severity, Span};

/// Format a single diagnostic for terminal display.
///
/// `source` — the full source text of the file (used for inline context).
/// `filename` — displayed in the `-->` location line (defaults to `<input>`).
pub fn format_diagnostic(diag: &Diagnostic, source: Option<&str>, filename: Option<&str>) -> String {
    let mut out = String::new();

    // Header: severity[code]: message
    out.push_str(&format!("{}[{}]: {}", diag.severity, diag.code, diag.message));

    // Location line
    if let Some(span) = &diag.span {
        let file = filename.unwrap_or("<input>");
        out.push_str(&format!("\n  --> {}:{}", file, span.start));
    }

    // Primary source context with underline
    if let (Some(source), Some(span)) = (source, &diag.span) {
        if let Some(context) = format_source_context(source, span) {
            out.push('\n');
            out.push_str(&context);
        }
    }

    // Secondary labels
    if let Some(source) = source {
        for label in &diag.labels {
            out.push_str(&format!("\n  --> {}", label.span.start));
            if let Some(context) = format_source_context(source, &label.span) {
                out.push('\n');
                out.push_str(&context);
            }
            out.push_str(&format!("\n  = {}", label.message));
        }
    }

    // Help text
    if let Some(help) = &diag.help {
        out.push_str(&format!("\n  = help: {}", help));
    }

    // Fix suggestion (description only; edits are applied programmatically)
    if let Some(fix) = &diag.fix {
        out.push_str(&format!("\n  = suggestion: {}", fix.description));
    }

    out
}

/// Print all diagnostics to stderr, one per entry, with a blank line between each.
pub fn print_diagnostics(diags: &[Diagnostic], source: &str, path: &Path) {
    let filename = path.to_string_lossy();
    for (i, d) in diags.iter().enumerate() {
        if i > 0 {
            eprintln!();
        }
        eprintln!("{}", format_diagnostic(d, Some(source), Some(&filename)));
    }
}

/// Print a summary line: `N error(s), M warning(s)`.
pub fn print_summary(diags: &[Diagnostic]) {
    let errors = diags.iter().filter(|d| d.severity == Severity::Error).count();
    let warnings = diags.iter().filter(|d| d.severity == Severity::Warning).count();

    if errors == 0 && warnings == 0 {
        return;
    }

    let mut parts = Vec::new();
    if errors > 0 {
        parts.push(format!("{} error{}", errors, if errors == 1 { "" } else { "s" }));
    }
    if warnings > 0 {
        parts.push(format!("{} warning{}", warnings, if warnings == 1 { "" } else { "s" }));
    }
    eprintln!("{}", parts.join(", "));
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn format_source_context(source: &str, span: &Span) -> Option<String> {
    let lines: Vec<&str> = source.lines().collect();
    let line_idx = span.start.line.checked_sub(1)?;

    if line_idx >= lines.len() {
        return None;
    }

    let line = lines[line_idx];
    let line_num = span.start.line;
    let gutter_width = format!("{}", line_num).len();

    let mut out = String::new();

    // Blank gutter line
    out.push_str(&format!("{:>width$} |\n", "", width = gutter_width));

    // Source line
    out.push_str(&format!("{:>width$} | {}\n", line_num, line, width = gutter_width));

    // Underline
    let col_start = span.start.column.saturating_sub(1);
    let col_end = if span.start.line == span.end.line {
        span.end.column.saturating_sub(1).max(col_start + 1)
    } else {
        line.len()
    };
    let underline_len = col_end.saturating_sub(col_start).max(1);

    out.push_str(&format!(
        "{:>width$} | {:>pad$}{}",
        "",
        "",
        "^".repeat(underline_len),
        width = gutter_width,
        pad = col_start,
    ));

    Some(out)
}
