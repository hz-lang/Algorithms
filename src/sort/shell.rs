//! 希尔排序。

fn sort(a: &mut [i32]) {
    let mut h = 1;
    while h < a.len() / 3 {
        h = 3 * h + 1;
    }
    while h >= 1 {
        for i in h..a.len() {
            let mut j = i;
            while j >= h && a[j] < a[j - h] {
                a.swap(j, j - h);
                j -= h;
            }
        }
        h /= 3;
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::{is_sorted, shell::sort};

    #[test]
    fn test() {
        let mut a = [8, 5, 7, 6, 4, 9, 3, 1];
        sort(&mut a);
        assert!(is_sorted(&a));
    }
}
