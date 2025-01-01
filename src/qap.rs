use lambdaworks_math::field::{element::FieldElement, traits::IsPrimeField};
use lambdaworks_math::polynomial::Polynomial;

use crate::R1CS;

pub struct QAP<F: IsPrimeField> {
    pub a: Vec<Polynomial<FieldElement<F>>>,
    pub b: Vec<Polynomial<FieldElement<F>>>,
    pub c: Vec<Polynomial<FieldElement<F>>>,
    pub xs: Vec<FieldElement<F>>,
    pub t: Polynomial<FieldElement<F>>,
}

impl<F: IsPrimeField> From<&R1CS<F>> for QAP<F> {
    fn from(r1cs: &R1CS<F>) -> Self {
        // create random variables for each constraint
        let xs = (1..=r1cs.num_constraints())
            .map(|i| FieldElement::<F>::from(i as u64))
            .collect::<Vec<_>>();

        // interpolate polynomials
        let [a, b, c] = [&r1cs.a, &r1cs.b, &r1cs.c].map(|mat| {
            (0..r1cs.num_variables())
                .map(|j| {
                    let mut ys = Vec::with_capacity(r1cs.num_constraints());
                    for i in 0..r1cs.num_constraints() {
                        ys.push(mat[i][j].clone());
                    }

                    Polynomial::interpolate(&xs, &ys).expect("should interpolate")
                })
                .collect::<Vec<_>>()
        });

        // build target polynomial T(x) = (x - x1)(x - x2)...(x - xn)
        let t = xs
            .iter()
            .map(|f| Polynomial::new(&[-f, FieldElement::one()]))
            .reduce(|acc, p| acc * p)
            .unwrap();

        Self { a, b, c, xs, t }
    }
}

impl<F: IsPrimeField> From<R1CS<F>> for QAP<F> {
    fn from(r1cs: R1CS<F>) -> Self {
        Self::from(&r1cs)
    }
}

impl<F: IsPrimeField> QAP<F> {
    /// Return the number of variables in the QAP, indicated by the maximum degree.
    #[inline]
    pub fn num_variables(&self) -> usize {
        self.a.len()
    }

    /// Return the number of constraints in the QAP.
    #[inline]
    pub fn num_constraints(&self) -> usize {
        let deg_a = self.a.iter().map(Polynomial::degree).max().unwrap_or(0);
        let deg_b = self.b.iter().map(Polynomial::degree).max().unwrap_or(0);
        let deg_c = self.c.iter().map(Polynomial::degree).max().unwrap_or(0);
        deg_a.max(deg_b).max(deg_c) + 1
    }

    /// Builds P(x) for a given assignment w.
    pub fn build_p(&self, w: &[FieldElement<F>]) -> Polynomial<FieldElement<F>> {
        assert!(w.len() == self.num_variables());

        // calcualte sums of a, b, c separately
        let [p_a, p_b, p_c] = [&self.a, &self.b, &self.c].map(|poly| {
            (0..self.num_variables()).fold(Polynomial::zero(), |acc, i| {
                acc + poly[i].scale_coeffs(&w[i])
            })
        });

        // A * B - C
        p_a * p_b - p_c
    }

    /// Checks if P(x) is divisible by T(x).
    pub fn divide_by_t(
        &self,
        p: Polynomial<FieldElement<F>>,
    ) -> (Polynomial<FieldElement<F>>, Polynomial<FieldElement<F>>) {
        p.long_division_with_remainder(&self.t)
    }

    pub fn evaluate_over_domain(&self, p: &Polynomial<FieldElement<F>>) -> FieldElement<F> {
        self.xs
            .iter()
            .fold(FieldElement::zero(), |acc, x| acc + p.evaluate(&x))
    }
}
