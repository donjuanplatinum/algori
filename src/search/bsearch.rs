use core::cmp::Ordering;

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
