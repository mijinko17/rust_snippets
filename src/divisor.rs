use cargo_snippet::snippet;

#[snippet("divisor")]
fn divisor<T: num::PrimInt>(a: T) -> Vec<T> {
    let div_small = num::iter::range_inclusive(T::one(), a)
        .take_while(|&i| i * i <= a)
        .filter(|&i| T::is_zero(&(a % i)));
    let div_large = num::iter::range_inclusive(T::one(), a)
        .take_while(|&i| i * i < a)
        .filter(|&i| T::is_zero(&(a % i)))
        .map(|i| a / i);
    div_small.chain(div_large).collect()
}
