use cargo_snippet::snippet;

#[snippet("CombinationMod")]
pub struct CombinationMod {
    fac: Vec<i64>,
    inv: Vec<i64>,
    inv_fac: Vec<i64>,
    modulo: i64,
}

#[snippet("CombinationMod")]
impl CombinationMod {
    pub fn new(size: usize, modulo: i64) -> CombinationMod {
        let mut fac = vec![1; size + 1];
        let mut inv = vec![1; size + 1];
        let mut inv_fac = vec![1; size + 1];
        for i in 2..=size {
            fac[i] = fac[i - 1] * i as i64 % modulo;
            inv[i] = modulo - inv[modulo as usize % i] * (modulo / i as i64) % modulo;
            inv_fac[i] = inv_fac[i - 1] * inv[i] % modulo;
        }
        CombinationMod {
            fac: fac,
            inv: inv,
            inv_fac: inv_fac,
            modulo: modulo,
        }
    }
    pub fn query(&self, n: i64, r: i64) -> i64 {
        self.fac[n as usize] * self.inv_fac[r as usize] % self.modulo
            * self.inv_fac[(n - r) as usize]
            % self.modulo
    }
    pub fn inverse(&self, n: i64) -> i64 {
        self.inv[n as usize]
    }
}

#[test]
fn test_combination_mod() {
    let modulo = 1000000007;
    let comb = CombinationMod::new(100, modulo);
    assert_eq!(comb.query(7, 3), 35);
    assert_eq!(comb.query(32, 16), 601080390);
    assert_eq!(comb.query(5, 0), 1);
    assert_eq!(comb.query(13, 13), 1);
    for i in 1..100 {
        assert_eq!(comb.inverse(i) * i % modulo, 1);
    }
}
