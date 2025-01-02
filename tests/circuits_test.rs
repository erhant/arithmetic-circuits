use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField;
use mpbc_arithmetic_circuits::*;

const ORDER: u64 = 97;
type F = U64PrimeField<ORDER>; // Example field with a small prime for simplicity
type W = circuits::Wire<F>;

#[test]
fn test_main() {
    let a = W::new(23, "a".to_string());
    let b = W::new(23, "b".to_string());
    let c = W::new(55, "c".to_string());

    let a_is_b = circuits::comparators::is_equal(&a, &b);
    println!("a == b is given by {}", a_is_b);

    let b_is_c = circuits::comparators::is_equal(&b, &c);
    println!("b == c is given by {}", b_is_c);

    // create r1cs
    let cc = c.clone() * c.clone();
    let ccc = cc * c;
    println!("c^3 is given by {}", ccc);
}
