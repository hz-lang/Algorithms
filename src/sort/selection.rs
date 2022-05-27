//! 选择排序。

fn sort(a: &mut [i32]) {
    for i in 0..a.len() {
        let mut min = i;
        for j in i + 1..a.len() {
            if a[j] < a[min] {
                min = j;
            }
        }
        a.swap(i, min);
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::{is_sorted, selection::sort};
    
    #[test]
    fn test() {
        let mut a = [8, 5, 7, 6, 4, 9, 3, 1];
        sort(&mut a);
        assert!(is_sorted(&a));
    }
}
