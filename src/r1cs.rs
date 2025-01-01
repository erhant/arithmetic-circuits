use super::dot;
use lambdaworks_math::field::{element::FieldElement, traits::IsField};

/// A representation of a Rank-1 Constraint System (R1CS) over a finite-field `F`.
pub struct R1CS<F: IsField> {
    pub a: Vec<Vec<FieldElement<F>>>,
    pub b: Vec<Vec<FieldElement<F>>>,
    pub c: Vec<Vec<FieldElement<F>>>,
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

        Self { a, b, c }
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

    // TODO: add "print_constraint" method that prints a constraint w.r.t some variable names
}
