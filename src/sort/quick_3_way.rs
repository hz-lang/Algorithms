//! 三向切分的快速排序。

fn sort(a: &mut [i32]) {
    sort_internal(a);
}

fn sort_internal(a: &mut [i32]) {
    if a.len() < 2 {
        return;
    }
    let mut lt = 0;
    let mut i = 1;
    let mut gt = a.len() - 1;
    let v = a[0]; // 切分元素。
    while i <= gt {
        if a[i] == v {
            i += 1;
        } else if a[i] > v {
            a.swap(i, gt);
            gt -= 1;
        } else {
            a.swap(lt, i);
            lt += 1;
            i += 1;
        }
    }
    sort_internal(&mut a[..lt]);
    sort_internal(&mut a[gt + 1..]); // 排序右半边，一定不要包含切分元素。
}

#[cfg(test)]
mod tests {
    use crate::sort::{is_sorted, quick_3_way::sort};

    #[test]
    fn test() {
        let mut a = [8, 5, 7, 6, 4, 9, 3, 1];
        sort(&mut a);
        assert!(is_sorted(&a));
    }
}
