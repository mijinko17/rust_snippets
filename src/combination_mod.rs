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
    pub fn comb(&self, n: i64, r: i64) -> i64 {
        let (n, r) = (n as usize, r as usize);
        self.fac[n] * self.inv_fac[r] % self.modulo * self.inv_fac[n - r] % self.modulo
    }
    pub fn perm(&self, n: i64, r: i64) -> i64 {
        let (n, r) = (n as usize, r as usize);
        self.fac[n] * self.inv_fac[n - r] % self.modulo
    }
    pub fn inverse(&self, n: i64) -> i64 {
        self.inv[n as usize]
    }
}

#[test]
fn test_combination_mod_comb() {
    let modulo = 1000000007;
    let comb = CombinationMod::new(100, modulo);
    assert_eq!(comb.comb(7, 3), 35);
    assert_eq!(comb.comb(32, 16), 601080390);
    assert_eq!(comb.comb(5, 0), 1);
    assert_eq!(comb.comb(13, 13), 1);
}

#[test]
fn test_combination_mod_perm() {
    let modulo = 1000000007;
    let comb = CombinationMod::new(100, modulo);
    for n in 0..100 {
        for r in 0..=n {
            let n_p_r = ((n - r + 1)..=n).fold(1, |acc, x| acc * x % modulo);
            assert_eq!(comb.perm(n, r), n_p_r);
        }
    }
}

#[test]
fn test_combination_mod_inverse() {
    let modulo = 1000000007;
    let comb = CombinationMod::new(100, modulo);
    for i in 1..100 {
        assert_eq!(comb.inverse(i) * i % modulo, 1);
    }
}
