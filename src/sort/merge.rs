//! 自顶向下的归并排序。

use std::usize;

fn sort(a: &mut [i32]) {
    let mut aux = vec![0; a.len()];
    sort_internal(&mut aux[..], a, 0, a.len());
}

fn sort_internal(aux: &mut [i32], a: &mut [i32], lo: usize, hi: usize) {
    if lo + 1 >= hi {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    sort_internal(aux, a, lo, mid);
    sort_internal(aux, a, mid, hi);
    merge(aux, a, lo, mid, hi);
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
    use crate::sort::{is_sorted, merge::sort};

    #[test]
    fn test() {
        let mut a = [8, 5, 7, 6, 4, 9, 3, 1];
        sort(&mut a);
        assert!(is_sorted(&a));
    }
}
