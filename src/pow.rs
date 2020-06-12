use cargo_snippet::snippet;

#[snippet("modpow")]
fn modpow(a: i64, x: i64, m: i64) -> i64 {
    if x == 0 {
        1
    } else if x % 2 != 0 {
        a * modpow(a, x - 1, m) % m
    } else {
        let half = modpow(a, x / 2, m);
        half * half % m
    }
}

#[snippet("intpow")]
fn intpow(a: i64, x: i64) -> i64 {
    if x == 0 {
        1
    } else if x % 2 != 0 {
        a * intpow(a, x - 1)
    } else {
        let half = intpow(a, x / 2);
        half * half
    }
}
