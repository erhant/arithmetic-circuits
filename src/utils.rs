use lambdaworks_math::{
    field::{
        element::FieldElement,
        traits::{IsField, IsPrimeField},
    },
    polynomial::Polynomial,
};

/// Compute the dot product of two vectors over the same field.
#[inline]
pub fn dot<F: IsField>(a: &[FieldElement<F>], b: &[FieldElement<F>]) -> FieldElement<F> {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Pretty-prints a polynomial
pub fn poly_print<F: IsPrimeField>(poly: &Polynomial<FieldElement<F>>) -> String {
    let coeff_decimals = poly
        .coefficients()
        .iter()
        .map(|coeff| format!("{}", coeff.representative()))
        .collect::<Vec<_>>();

    let result = coeff_decimals
        .iter()
        .enumerate()
        .rev()
        .map(|(i, coeff)| match (i, coeff.as_str()) {
            (_, "0") => String::new(),
            (0, _) => coeff.to_string(),
            (1, "1") => "x".to_string(),
            (1, _) => format!("{}*x", coeff),
            (_, "1") => format!("x^{}", i),
            (_, _) => format!("{}*x^{}", coeff, i),
        })
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" + ");

    if result.is_empty() {
        "0".to_string()
    } else {
        result
    }
}
