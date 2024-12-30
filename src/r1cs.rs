use lambdaworks_math::field::fields::u64_prime_field::U64PrimeField;
use lambdaworks_math::field::{element::FieldElement, traits::IsField};

/// A representation of a Rank-1 Constraint System (R1CS) over a finite-field `F`.
pub struct R1CS<F: IsField> {
    a: Vec<Vec<FieldElement<F>>>,
    b: Vec<Vec<FieldElement<F>>>,
    c: Vec<Vec<FieldElement<F>>>,
}

impl<F: IsField> R1CS<F> {
    pub fn new(
        a: Vec<Vec<FieldElement<F>>>,
        b: Vec<Vec<FieldElement<F>>>,
        c: Vec<Vec<FieldElement<F>>>,
    ) -> Self {
        // assert that all lengths are equal
        assert!(a.len() == b.len() && b.len() == c.len());
        for i in 0..a.len() {
            assert!(a[i].len() == b[i].len() && b[i].len() == c[i].len());
        }

        R1CS { a, b, c }
    }

    /// Return the number of constraints in the R1CS.
    #[inline]
    pub fn num_constraints(&self) -> usize {
        self.a.len()
    }

    /// Return the number of variables in the R1CS.
    #[inline]
    pub fn num_variables(&self) -> usize {
        self.a[0].len()
    }

    /// Check if the given assignment satisfies the R1CS.
    pub fn satisfies(&self, x: &[FieldElement<F>]) -> bool {
        let (a, b, c) = (&self.a, &self.b, &self.c);
        (0..self.num_constraints()).all(|i| {
            // Ax * Bx == Cx, for all constraints
            dot(&a[i], x) * dot(&b[i], x) == dot(&c[i], x)
        })
    }
}

/// Compute the dot product of two vectors over the same field.
#[inline]
fn dot<F: IsField>(a: &[FieldElement<F>], b: &[FieldElement<F>]) -> FieldElement<F> {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    const ORDER: u64 = 97;
    type F = U64PrimeField<ORDER>; // Example field with a small prime for simplicity
    type FE = FieldElement<F>;

    const NUM_CONSTRAINTS: usize = 3;
    const NUM_VARIABLES: usize = 5;

    #[rustfmt::skip]
    const A: [[u64; NUM_VARIABLES]; NUM_CONSTRAINTS] = [
        [       0, 1, 0, 0, 0],
        [       0, 0, 1, 0, 0],
        [ORDER-20, 1, 0, 1, 0]
    ];

    #[rustfmt::skip]
    const B: [[u64; NUM_VARIABLES]; NUM_CONSTRAINTS] = [
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0]
    ];

    #[rustfmt::skip]
    const C: [[u64; NUM_VARIABLES]; NUM_CONSTRAINTS] = [
        [0, 0, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [0, 0, 0, 0, 1]
    ];

    // x = 3, [1, x, w1, w2, w3] = [1, 3, 9, 27, 0]
    const X: [u64; NUM_VARIABLES] = [1, 2, 3, 4, 5];

    #[test]
    fn test_r1cs() {
        let r1cs = R1CS::new(
            A.into_iter()
                .map(|a| a.into_iter().map(|aa| FE::from(aa)).collect())
                .collect(),
            B.into_iter()
                .map(|b| b.into_iter().map(|bb| FE::from(bb)).collect())
                .collect(),
            C.into_iter()
                .map(|c| c.into_iter().map(|cc| FE::from(cc)).collect())
                .collect(),
        );

        assert_eq!(r1cs.num_constraints(), NUM_CONSTRAINTS);
        assert_eq!(r1cs.num_variables(), NUM_VARIABLES);

        // check that the assignment satisfies the R1CS
        assert!(r1cs.satisfies(&X.iter().map(|x| FE::from(*x)).collect::<Vec<_>>()));
    }
}
