use lambdaworks_math::{
    field::{element::FieldElement, fields::u64_prime_field::U64PrimeField},
    polynomial::Polynomial,
};
use mpbc_arithmetic_circuits::*;

const ORDER: u64 = 97;
type F = U64PrimeField<ORDER>; // Example field with a small prime for simplicity
type FE = FieldElement<F>;
type W = circuits::Wire<F>;

#[test]
fn test_main() {
    let a = W::from(23);
    let b = W::from(23);
    let c = W::from(55);

    let a_is_b = circuits::comparators::is_equal(&a, &b);
    println!("a == b: {}", a_is_b);

    // create r1cs
    let cc = c * c;
    let ccc = c * c * c;
    println!("c^3 = {}", cc);

    // check that the assignment satisfies the R1CS
    let x = X.iter().map(|x| FE::from(*x)).collect::<Vec<_>>();
    assert!(r1cs.satisfies(&x), "r1cs is not satisfied");

    // create QAP
    let qap = QAP::from(&r1cs);
    assert_eq!(qap.num_constraints(), NUM_CONSTRAINTS);
    assert_eq!(qap.num_variables(), NUM_VARIABLES);

    // target poly should evaluate to zero over xs
    assert_eq!(
        qap.evaluate_over_domain(&qap.t),
        FE::zero(),
        "target poly evaluation is not zero"
    );

    println!("A:");
    for (i, a_i) in qap.a.iter().enumerate() {
        println!("\tA_{}(x) = {}", i, poly_print(a_i));
        for xs_i in qap.xs.iter() {
            print!(
                "\tA_{}({}) = {}",
                i,
                xs_i.representative(),
                a_i.evaluate(xs_i).representative()
            );
        }
        println!("");
    }

    // check that the assignment satisfies the QAP
    let p = qap.build_p(&x);
    println!("P(x) = {}", poly_print(&p));

    // evaluate the QAP over its own assignment
    let eval = qap.evaluate_over_domain(&p);
    assert_eq!(eval, FE::zero(), "QAP evaluation is not zero");

    // check that P(x) is divisible by T(x)
    let (quotient, remainder) = qap.divide_by_t(p);
    assert_eq!(remainder, Polynomial::zero(), "remainder is not zero");
    println!("P(x) / T(x) = {}", poly_print(&quotient));
}
