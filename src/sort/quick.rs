//! 快速排序。

fn sort(a: &mut [i32]) {
    sort_internal(a);
}

fn sort_internal(a: &mut [i32]) {
    if a.len() < 2 {
        return;
    }
    let j = partition(a);
    sort_internal(&mut a[..j]);
    sort_internal(&mut a[j + 1..]); // 排序右半边，一定不要包含切分元素。
}

fn partition(a: &mut [i32]) -> usize {
    debug_assert!(a.len() > 1);

    let mut i = 1;
    let mut j = a.len() - 1;
    let v = a[0]; // 切分元素。
    loop {
        // i 加到 a.len() - 1 即可。
        while i < j && a[i] < v {
            i += 1;
        }
        // 切分元素本身即哨兵，因此 j 不可能越界。
        while a[j] > v {
            j -= 1;
        }
        if i >= j {
            break;
        }
        a.swap(i, j);
    }
    a.swap(0, j);
    j
}

#[cfg(test)]
mod tests {
    use crate::sort::{is_sorted, quick::sort};

    #[test]
    fn test() {
        let mut a = [8, 5, 7, 6, 4, 9, 3, 1];
        sort(&mut a);
        assert!(is_sorted(&a));
    }
}
