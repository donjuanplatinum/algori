use core::cmp::Ordering;

/// # Binary Search
///
/// Binary search efficiently locates an element in a sorted slice by repeatedly
/// dividing the search range in half. At each iteration, the middle element is
/// compared with the search key, and the half that cannot contain the key is
/// discarded.
///
/// - Return Ok(idx) if the element is found.
/// - Return Err(idx) if the key is absent. And the `idx` is the insertion position
/// that preserves the sorted order may be used in other algorithms.
///
/// ## Complexity
///
/// - Time: $O(log n)$, where n is the slice length.
/// - Space: $O(1)$, excluding any allocations performed by the comparator.
#[inline]
pub fn bsearch<T, F>(slice: &[T], key: &T, cmp: F) -> Result<usize, usize>
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut num = slice.len();
    let mut base = 0;
    while num > 0 {
        let pivot = base + (num >> 1);
        match cmp(key, &slice[pivot]) {
            Ordering::Equal => {
                return Ok(pivot);
            }
            Ordering::Greater => {
                base = pivot + 1;
                num -= 1;
            }
            _ => {}
        }
        num >>= 1;
    }
    Err(base)
}

#[cfg(test)]
mod tests {
    use crate::search::bsearch;
    macro_rules! test_search {
        ($slice:expr,$key:expr,$expect:expr) => {
            let res = bsearch(&$slice, &$key, |a, b| a.cmp(b));
            assert_eq!(res, $expect);
        };
    }
    #[test]
    fn bsearch_test() {
        let arr = [0, 1, 10, 20, 30, 100];
        test_search!(arr, 10, Ok(2));
        test_search!(arr, 0, Ok(0));
        test_search!(arr, -1, Err(0));
        test_search!(arr, 2, Err(2));
        test_search!(arr, 100, Ok(5));
        test_search!(arr, 101, Err(6));
    }
}
