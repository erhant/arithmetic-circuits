use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField;
use mpbc_arithmetic_circuits::*;

// some arbitrary field order
const ORDER: u64 = 97;
type F = U64PrimeField<ORDER>;
type W = circuits::Wire<F>;

#[test]
fn test_is_equal() {
    let a = W::new(23, "a");
    let b = W::new(23, "b");
    let c = W::new(55, "c");

    let a_is_b = circuits::comparators::is_equal(&a, &b);
    assert_eq!(a_is_b.label, "(1+(96*((a+(96*b))*0)))");

    let b_is_c = circuits::comparators::is_equal(&b, &c);
    assert_eq!(b_is_c.label, "(1+(96*((b+(96*c))*(b+(96*c))_neg)))");
}

#[test]
fn test_c_cube() {
    let c = W::new(55, "c");
    let cc = &c * &c;
    let ccc = cc * c;

    assert_eq!(ccc.label, "((c*c)*c)");
    assert_eq!(ccc.value, (55 * 55 * 55).into());
}

#[test]
fn test_vitalik_example() {
    // we are constructing: x^3 + x + 5 = 35
    let x = W::new(3, "x");
    let xx = &x * &x;
    let ans = (&xx * &x) + x + 5.into();

    assert_eq!(ans.label, "((((x*x)*x)+x)+5)");
    assert_eq!(ans.value, 35.into());
}
