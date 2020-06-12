use cargo_snippet::snippet;

#[snippet("ExtGcd")]
#[snippet("InvMod")]
fn ext_gcd<T>(a: T, b: T) -> (T, T)
where
    T: num::Num + num::Signed + Copy,
{
    if T::is_zero(&b) {
        (T::one(), T::zero())
    } else {
        let (y_before, x_before) = ext_gcd(b, a % b);
        (x_before, y_before - a / b * x_before)
    }
}

#[snippet("InvMod")]
fn inv_mod<T>(a: T, modulo: T) -> T
where
    T: num::Num + num::Signed + Copy,
{
    (ext_gcd(a % modulo, modulo).0 + modulo) % modulo
}

#[test]
fn test_ext_gcd() {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    for a in 0..100 {
        for b in 0..100 {
            let (x, y) = ext_gcd(a, b);
            assert_eq!(a * x + b * y, gcd(a, b));
        }
    }
}

#[test]
fn test_inv_mod() {
    let modulo: i64 = 1000000007;
    for a in 1..100 {
        let inv = inv_mod(a, modulo);
        let res = (a * inv) % modulo;
        assert_eq!(res, 1);
    }
    for a in modulo - 100..modulo {
        let inv = inv_mod(a, modulo);
        let res = (a * inv) % modulo;
        assert_eq!(res, 1);
    }
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    for a in 1..100 {
        for m in 2..100 {
            if gcd(a, m) == 1 {
                println!("{} {}", a, m);
                let inv = inv_mod(a, m);
                let res = (a * inv) % m;
                assert_eq!(res, 1);
            }
        }
    }
}
