//! Small string-distance helpers shared across diagnostic producers.

/// Classic iterative Levenshtein edit distance over Unicode scalar values.
///
/// Used to back "did you mean …?" suggestions when a fuzzy/subsequence
/// matcher returns nothing (e.g. transposition typos like `wieght` →
/// `weight`, which a subsequence match cannot catch).
pub fn levenshtein(a: &str, b: &str) -> usize {
    let b_chars: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b_chars.len()).collect();
    let mut curr = vec![0usize; b_chars.len() + 1];
    for (i, ca) in a.chars().enumerate() {
        curr[0] = i + 1;
        for (j, &cb) in b_chars.iter().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            curr[j + 1] = (prev[j + 1] + 1).min(curr[j] + 1).min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[b_chars.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_basics() {
        assert_eq!(levenshtein("", ""), 0);
        assert_eq!(levenshtein("abc", "abc"), 0);
        assert_eq!(levenshtein("abc", ""), 3);
        // Single transposition costs two edits under plain Levenshtein.
        assert_eq!(levenshtein("wieght", "weight"), 2);
        // Unicode scalar values, not bytes.
        assert_eq!(levenshtein("café", "cafe"), 1);
    }
}
