//! 排序算法。

mod selection;
mod insertion;
mod shell;
mod merge;
mod merge_bu;
mod quick;
mod quick_3_way;
mod binary_heap;
mod heap_sort;
mod max_pq;
pub(crate) mod min_pq;

fn is_sorted(a: &[i32]) -> bool {
    for i in 1..a.len() {
        if a[i - 1] > a[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::is_sorted;

    #[test]
    fn test() {
        let a = [1, 2, 3];
        assert!(is_sorted(&a));

        let b = [2, 1, 3];
        assert!(!is_sorted(&b));
    }
}
