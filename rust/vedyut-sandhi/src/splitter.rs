/// Sandhi splitting - reverse sandhi to find original words

/// Split a sandhi-combined word into possible original components
///
/// # Arguments
/// * `text` - Sandhi-combined text
///
/// # Returns
/// List of possible splits, each as (left, right) tuple
pub fn split_sandhi(text: &str) -> Vec<(String, String)> {
    // TODO: Implement actual sandhi splitting
    // This requires reverse-engineering sandhi rules

    // Placeholder: return simple character-based splits
    let mut results = Vec::new();

    // Iterate over char boundaries, skipping first and last (trivial splits)
    for (i, _) in text.char_indices().skip(1) {
        let left = &text[..i];
        let right = &text[i..];
        results.push((left.to_string(), right.to_string()));
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_sandhi_placeholder() {
        let splits = split_sandhi("धर्मक्षेत्रे");
        assert!(!splits.is_empty());
    }
}
