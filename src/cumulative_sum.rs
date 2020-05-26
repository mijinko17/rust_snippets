use cargo_snippet::snippet;

#[snippet("CumulativeSum")]
struct CumulativeSum {
    cum: Vec<i64>,
}

#[snippet("CumulativeSum")]
impl CumulativeSum {
    pub fn from_vec(v: &Vec<i64>) -> Self {
        CumulativeSum {
            cum: vec![0]
                .iter()
                .chain(v.iter())
                .scan(0, |acc, &x| {
                    *acc += x;
                    Some(*acc)
                })
                .collect(),
        }
    }
    pub fn query(&self, a: usize, b: usize) -> i64 {
        self.cum[b] - self.cum[a]
    }
}

#[test]
fn test_cumulative_sum() {
    let v = vec![4, 1, -5, 3, 10];
    let cum: CumulativeSum = CumulativeSum::from_vec(&v);
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
