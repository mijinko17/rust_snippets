use cargo_snippet::snippet;

#[snippet("CumulativeSum")]
#[derive(Debug, Clone)]
struct CumulativeSum<T: num::Num + Copy> {
    cum: Vec<T>,
}

#[snippet("CumulativeSum")]
impl<T: num::Num + Copy> CumulativeSum<T> {
    pub fn from_vec(v: &Vec<T>) -> Self {
        v.iter().copied().collect()
    }
    pub fn query(&self, a: usize, b: usize) -> T {
        self.cum[b] - self.cum[a]
    }
}

#[snippet("CumulativeSum")]
impl<T: num::Num + Copy> std::iter::FromIterator<T> for CumulativeSum<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        CumulativeSum {
            cum: (std::iter::once(T::zero()).chain(iter))
                .scan(T::zero(), |acc, x| {
                    *acc = *acc + x;
                    Some(*acc)
                })
                .collect(),
        }
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
