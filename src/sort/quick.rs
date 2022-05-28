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
        while i + 1 < a.len() && a[i] <= v {
            i += 1;
        }
        // j 减到 1 即可，再减就是切分元素了。
        while j > 1 && a[j] > v {
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
