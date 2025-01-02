use lambdaworks_math::{
    field::{element::FieldElement, fields::u64_prime_field::U64PrimeField},
    polynomial::Polynomial,
};
use mpbc_arithmetic_circuits::*;

const ORDER: u64 = 97;
type F = U64PrimeField<ORDER>; // Example field with a small prime for simplicity
type FE = FieldElement<F>;

const NUM_CONSTRAINTS: usize = 3;
const NUM_VARIABLES: usize = 4;

#[rustfmt::skip]
    const A: [[u64; NUM_VARIABLES]; NUM_CONSTRAINTS] = [
     //  1  x  w1 w2
        [0, 1, 0, 0],
        [0, 0, 1, 0],
        [5, 1, 0, 1]
    ];

#[rustfmt::skip]
    const B: [[u64; NUM_VARIABLES]; NUM_CONSTRAINTS] = [
     //  1  x  w1 w2
        [0, 1, 0, 0],
        [0, 1, 0, 0],
        [0, 0, 0, 0]
    ];

#[rustfmt::skip]
    const C: [[u64; NUM_VARIABLES]; NUM_CONSTRAINTS] = [
     //  1  x  w1 w2
        [0, 0, 1, 0],
        [0, 0, 0, 1],
        [0, 0, 0, 0]
    ];

// x = 3 results in variables: [1, x, w1, w2] = [1, 3, 9, 27]
const X: [u64; NUM_VARIABLES] = [1, 3, 9, 27];

#[test]
fn test_main() {
    // convert A, B, C to field elements
    let [a, b, c] = [A, B, C].map(|mat| {
        mat.iter()
            .map(|mat_i| mat_i.iter().map(|mat_ij| FE::from(mat_ij)).collect())
            .collect()
    });

    // create r1cs
    let r1cs = R1CS::new(a, b, c);
    assert_eq!(r1cs.num_constraints(), NUM_CONSTRAINTS);
    assert_eq!(r1cs.num_variables(), NUM_VARIABLES);

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
