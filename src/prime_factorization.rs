use cargo_snippet::snippet;

#[snippet("PrimeFactorization")]
fn prime_factorization(x: i64) -> std::collections::BTreeMap<i64, i64> {
    (2..)
        .scan(x, |state, i| match *state {
            1 => None,
            _ => {
                let divisor = if i * i <= *state { i } else { *state };
                let mut index = 0;
                while *state % divisor == 0 {
                    *state /= divisor;
                    index += 1;
                }
                Some((divisor, index))
            }
        })
        .filter(|&(_, index)| index > 0)
        .collect()
}

#[test]
fn test_prime_factorization() {
    let pf_1024 = prime_factorization(1024);
    assert_eq!(pf_1024.get(&2), Some(&10));
    assert_eq!(pf_1024.get(&1), None);
    assert_eq!(pf_1024.get(&3), None);
    assert_eq!(pf_1024.get(&5), None);
    assert_eq!(pf_1024.get(&8), None);
    assert_eq!(pf_1024.get(&15), None);

    let pf_1 = prime_factorization(1);
    assert_eq!(pf_1.get(&3), None);
    assert_eq!(pf_1.get(&11), None);

    let pf_612 = prime_factorization(612);
    assert_eq!(pf_612.get(&2), Some(&2));
    assert_eq!(pf_612.get(&3), Some(&2));
    assert_eq!(pf_612.get(&17), Some(&1));
}
