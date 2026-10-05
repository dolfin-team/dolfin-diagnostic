//! Small string-distance helpers shared across diagnostic producers.

/// Optimal string alignment distance (Levenshtein plus adjacent
/// transpositions) over Unicode scalar values.
///
/// Used to back "did you mean …?" suggestions when a fuzzy/subsequence
/// matcher returns nothing. A swapped pair (`Dgo` → `Dog`) costs one edit,
/// so short names with a transposition typo stay within the hint threshold.
pub fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    // Three rolling rows: i-2, i-1, i.
    let mut prev2 = vec![0usize; b.len() + 1];
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut curr = vec![0usize; b.len() + 1];
    for i in 0..a.len() {
        curr[0] = i + 1;
        for j in 0..b.len() {
            let cost = usize::from(a[i] != b[j]);
            curr[j + 1] = (prev[j + 1] + 1).min(curr[j] + 1).min(prev[j] + cost);
            if i > 0 && j > 0 && a[i] == b[j - 1] && a[i - 1] == b[j] {
                curr[j + 1] = curr[j + 1].min(prev2[j - 1] + 1);
            }
        }
        std::mem::swap(&mut prev2, &mut prev);
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_basics() {
        assert_eq!(edit_distance("", ""), 0);
        assert_eq!(edit_distance("abc", "abc"), 0);
        assert_eq!(edit_distance("abc", ""), 3);
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        // Adjacent transposition costs a single edit.
        assert_eq!(edit_distance("wieght", "weight"), 1);
        assert_eq!(edit_distance("Dgo", "Dog"), 1);
        // Unicode scalar values, not bytes.
        assert_eq!(edit_distance("café", "cafe"), 1);
    }
}
