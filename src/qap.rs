use lambdaworks_math::field::{element::FieldElement, traits::IsPrimeField};

pub struct R1CS<F: IsPrimeField> {
    a: Vec<Vec<FieldElement<F>>>,
    b: Vec<Vec<FieldElement<F>>>,
    c: Vec<Vec<FieldElement<F>>>,
}

impl<F: IsPrimeField> R1CS<F> {
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

    /// Compute the dot product of two vectors over the same field.
    fn dot_prod(a: &[FieldElement<F>], b: &[FieldElement<F>]) -> FieldElement<F> {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
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

    pub fn satisfies(&self, input: &[FieldElement<F>]) -> bool {
        (0..self.num_constraints()).all(|i| {
            // Ax * Bx == Cx, for all constraints
            Self::dot_prod(&self.a[i], input) * Self::dot_prod(&self.b[i], input)
                == Self::dot_prod(&self.c[i], input)
        })
    }
}
