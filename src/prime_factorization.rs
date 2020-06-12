use cargo_snippet::snippet;

//#[snippet("PrimeFactorization")]
//fn prime_factorization2(x: i64) -> std::collections::BTreeMap<i64, i64> {
//    (std::iter::once(2).chain((3..).step_by(2)))
//        .scan(x, |state, i| match *state {
//            1 => None,
//            _ => {
//                let divisor = if i * i <= *state { i } else { *state };
//                let mut index = 0;
//                while *state % divisor == 0 {
//                    *state /= divisor;
//                    index += 1;
//                }
//                Some((divisor, index))
//            }
//        })
//        .filter(|&(_, index)| index > 0)
//        .collect()
//}

#[snippet("PrimeFactorization")]
fn prime_factorization<T: num::PrimInt>(x: T) -> std::collections::BTreeMap<T, i64> {
    let two = T::from(2).unwrap();
    let three = T::from(3).unwrap();
    (std::iter::once(two).chain(num::iter::range_step(three, x, two)))
        .scan(x, |state, i| {
            if T::is_one(state) {
                None
            } else {
                let divisor = if i * i <= *state { i } else { *state };
                let mut index = 0;
                while T::is_zero(&(*state % divisor)) {
                    *state = *state / divisor;
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
