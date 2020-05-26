use cargo_snippet::snippet;

#[snippet("BinarySearch")]
#[snippet("LowerUpperBound")]
pub fn meguru_binary_search<T, F>(mut ok: T, mut no: T, mut is_valid: F, error: T) -> T
where
    T: std::convert::From<u8>
        + std::ops::Add<Output = T>
        + std::ops::Div<Output = T>
        + std::cmp::Ord
        + std::marker::Copy,
    F: FnMut(T) -> bool,
{
    while ok + error < no || no + error < ok {
        let mid = (ok + no) / T::from(2u8);
        if is_valid(mid) {
            ok = mid;
        } else {
            no = mid;
        }
    }
    ok
}

#[snippet("LowerUpperBound")]
pub trait LowerUpperBound<T: Ord> {
    fn lower_bound(&self, value: &T) -> usize;
    fn upper_bound(&self, value: &T) -> usize;
}

#[snippet("LowerUpperBound")]
impl<T> LowerUpperBound<T> for [T]
where
    T: Ord,
{
    fn lower_bound(&self, value: &T) -> usize {
        meguru_binary_search(self.len() as i64, -1, |x| self[x as usize] >= *value, 1) as usize
    }
    fn upper_bound(&self, value: &T) -> usize {
        meguru_binary_search(self.len() as i64, -1, |x| self[x as usize] > *value, 1) as usize
    }
}

#[test]
fn test_lower_upper_bound() {
    let v: Vec<i32> = vec![-3, -2, -2, 0, 1, 4, 4, 4, 5, 12, 199];

    assert_eq!(v.lower_bound(&-100), 0);
    assert_eq!(v.lower_bound(&-3), 0);
    assert_eq!(v.lower_bound(&-2), 1);
    assert_eq!(v.lower_bound(&4), 5);
    assert_eq!(v.lower_bound(&199), 10);
    assert_eq!(v.lower_bound(&2000), 11);

    assert_eq!(v.upper_bound(&-100), 0);
    assert_eq!(v.upper_bound(&4), 8);
    assert_eq!(v.upper_bound(&199), 11);
}
