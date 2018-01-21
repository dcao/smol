//! Functions which return string metrics regarding the difference between strings
//! See <http://ntz-develop.blogspot.com/2011/03/fuzzy-string-search.html>

use std::borrow::Cow;
use std::mem::{replace, swap};

/// Finds the edit distance between two strings. Popularly known as the Levenshtein
/// distance. The algorithm used is Enhanced Ukkonen.
///
/// # Arguments
/// * `a`: the first piece of text to be compared.
/// * `b`: the second piece of text to be compared.
/// * `threshold`: The maximum distance value. A lower distance value will improve performance
/// in extreme cases, at the cost of correctness.
///
/// # Examples
/// ```rust
/// # use smol::metrics::distance::ukkonen;
/// assert_eq!(1, ukkonen("string", "strin", 10));
/// assert_eq!(2, ukkonen("string", "btrin", 10));
/// assert_eq!(3, ukkonen("string", "brtin", 10));
/// assert_eq!(6, ukkonen("", "string", 10));
/// assert_eq!(3, ukkonen("", "string", 3));
/// ```
///
/// Algorithm from <http://berghel.net/publications/asm/asm.php>.
/// Ported from <https://github.com/sunesimonsen/ukkonen>.
pub fn ukkonen<'a, S>(a: S, b: S, threshold: usize) -> usize
where
    S: PartialEq + Into<Cow<'a, str>>
{
    if a == b {
        return 0;
    }

    let (a, b): (Vec<_>, Vec<_>) = (a.into().chars().collect(), b.into().chars().collect());

    let (a, b) = if a.len() > b.len() { (b, a) } else { (a, b) };

    let (mut al, mut bl) = (a.len(), b.len());

    // Trim suffixes that are the same
    while al > 0 && a[al - 1] == b[al - 1] {
        al -= 1;
        bl -= 1;
    }

    // Trim prefixes that are the same
    let mut ts = 0;

    while ts < al && a[ts] == b[ts] {
        ts += 1;
    }

    al -= ts;
    bl -= ts;

    if al == 0 {
        return if bl < threshold { bl } else { threshold };
    }

    let threshold = if bl < threshold { bl } else { threshold };
    let delta = bl - al;

    let zero_k = ((if al < threshold { al } else { threshold }) >> 1) + 2;
    let arr_len = delta + zero_k * 2 + 2;

    let mut crow = Vec::with_capacity(arr_len);
    let mut nrow = Vec::with_capacity(arr_len);
    for _ in 0..arr_len {
        crow.push(-1);
        nrow.push(-1);
    }

    let a = &a[ts..ts + al];
    let b = &b[ts..ts + bl];

    let mut i = 0;
    let condition_row = delta + zero_k;
    let endmax = condition_row << 1;
    loop {
        i += 1;

        swap(&mut crow, &mut nrow);

        let start = if i <= zero_k {
            -(i as isize) + 1
        } else {
            i as isize - (zero_k << 1) as isize + 1
        };
        let mut previous_cell;
        let mut current_cell = -1;
        let mut next_cell = if i <= zero_k {
            i as isize - 2
        } else {
            crow[(zero_k as isize + start) as usize]
        };
        let end = if i <= condition_row { i } else { endmax - i };

        if i <= zero_k {
            nrow[zero_k + i as usize] = -1;
        }

        for x in start..end as isize {
            let rix = (zero_k as isize + x) as usize;
            previous_cell = replace(&mut current_cell, replace(&mut next_cell, crow[rix + 1]));

            let t = current_cell + 1;
            let t = if t < previous_cell { previous_cell } else { t };
            let mut t = if t < next_cell { next_cell + 1 } else { t } as usize;

            while t < al && t as isize + x < bl as isize && a[t] == b[(t as isize + x) as usize] {
                t += 1;
            }

            nrow[rix] = t as isize;
        }

        if !(nrow[condition_row] < al as isize && i <= threshold) {
            break;
        }
    }

    i - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ukkonen_threshold() {
        assert_eq!(6, ukkonen("Ukkonen", "Levenshtein", 6));
    }

    #[test]
    fn ukkonen_correct() {
        assert_eq!(8, ukkonen("Ukkonen", "Levenshtein", 1000));
    }

    #[test]
    fn ukkonen_basic() {
        assert_eq!(1, ukkonen("Test", "test", 10));
    }
}
