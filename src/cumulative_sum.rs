use cargo_snippet::snippet;

#[snippet("CumulativeSum")]
struct CumulativeSum<T: num::Num + Copy> {
    cum: Vec<T>,
}

#[snippet("CumulativeSum")]
impl<T: num::Num + Copy> CumulativeSum<T> {
    pub fn from_vec(v: &Vec<T>) -> Self {
        CumulativeSum {
            cum: (std::iter::once(&T::zero()).chain(v.iter()))
                .scan(T::zero(), |acc, &x| {
                    *acc = *acc + x;
                    Some(*acc)
                })
                .collect(),
        }
    }
    pub fn query(&self, a: usize, b: usize) -> T {
        self.cum[b] - self.cum[a]
    }
}

#[test]
fn test_cumulative_sum() {
    let v = vec![4, 1, -5, 3, 10];
    let cum = CumulativeSum::from_vec(&v);
    let n = v.len();
    for i in 0..=n {
        for j in i..=n {
            let mut sum = 0;
            for k in i..j {
                sum += v[k];
            }
            assert_eq!(cum.query(i, j), sum);
        }
    }
}
