//! 自底向上的归并排序。

use std::usize;

fn sort(a: &mut [i32]) {
    let mut aux = vec![0; a.len()];
    let mut i = 1;
    while i < a.len() {
        let mut lo = 0;
        while lo < a.len() - i {
            let mid = lo + i;
            let hi = a.len().min(lo + i + i);
            let s = space(i);
            println!("{s}lo={lo}, mid={mid}, hi={hi}, i={i}");
            merge(&mut aux[..], a, lo, mid, hi);
            lo += i + i;
        }
        i += i;
    }
}

fn space(mut i: usize) -> String {
    let mut s = String::with_capacity(i);
    while i > 1 {
        s.push_str("  ");
        i -= 1;
    }
    s
}

fn merge(aux: &mut [i32], a: &mut [i32], lo: usize, mid: usize, hi: usize) {
    let mut i = lo;
    let mut j = mid;
    for k in lo..hi {
        aux[k] = a[k];
    }

    for k in lo..hi {
        a[k] = if i >= mid {
            j += 1;
            aux[j - 1]
        } else if j >= hi {
            i += 1;
            aux[i - 1]
        } else if aux[j] < aux[i] {
            j += 1;
            aux[j - 1]
        } else {
            i += 1;
            aux[i - 1]
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::{is_sorted, merge_bu::sort};

    #[test]
    fn test() {
        let mut a = [8, 5, 7, 6, 4, 9, 3, 1];
        sort(&mut a);
        assert!(is_sorted(&a));
    }
}
