//! Audio TUI parsing utilities.

/// Extract `sink_name` from module arguments string.
///
/// Handles various formats:
/// - `sink_name=value`
/// - `sink_name="quoted value"`
/// - `sink_name='single quoted'`
#[must_use]
pub fn extract_sink_name(args: &str) -> String {
    if let Some(start) = args.find("sink_name=") {
        let rest = &args[start + 10..];

        // Handle quoted values
        if let Some(inner) = rest.strip_prefix('"') {
            if let Some(end) = inner.find('"') {
                return inner[..end].to_string();
            }
        } else if let Some(inner) = rest.strip_prefix('\'')
            && let Some(end) = inner.find('\'')
        {
            return inner[..end].to_string();
        }

        // Unquoted value - ends at whitespace or end of string
        let end = rest
            .find(|c: char| c.is_whitespace() || c == '\t')
            .unwrap_or(rest.len());
        return rest[..end].to_string();
    }
    "combined".to_string()
}
