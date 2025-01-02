use lambdaworks_math::field::{
    element::FieldElement,
    traits::{IsField, IsPrimeField},
};

#[derive(Clone, Debug)]
pub struct Wire<F: IsField> {
    pub label: String,
    pub value: FieldElement<F>,
}

impl<F: IsPrimeField> Wire<F> {
    pub fn constant(value: FieldElement<F>) -> Self {
        Self {
            label: value.representative().to_string(),
            value,
        }
    }

    pub fn new(value: impl Into<FieldElement<F>>, label: String) -> Self {
        Self {
            label,
            value: value.into(),
        }
    }

    #[inline]
    pub fn one() -> Self {
        Self::constant(FieldElement::<F>::one())
    }

    #[inline]
    pub fn neg_one() -> Self {
        Self::constant(-FieldElement::<F>::one())
    }

    #[inline]
    pub fn zero() -> Self {
        Self::constant(FieldElement::<F>::zero())
    }
}

impl<F: IsField> PartialEq for Wire<F> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<F: IsField> Eq for &Wire<F> {}

impl<F: IsPrimeField> std::fmt::Display for Wire<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.label, self.value.representative())
    }
}

impl<F: IsField> std::ops::Add<Wire<F>> for Wire<F> {
    type Output = Wire<F>;

    fn add(self, other: Wire<F>) -> Wire<F> {
        &self + &other
    }
}

impl<'a, 'b, F: IsField> std::ops::Add<&'b Wire<F>> for &'a Wire<F> {
    type Output = Wire<F>;

    fn add(self, other: &'b Wire<F>) -> Wire<F> {
        Wire {
            label: format!("({}+{})", self.label, other.label),
            value: self.value.clone() + other.value.clone(),
        }
    }
}

impl<F: IsField> std::ops::Mul<Wire<F>> for Wire<F> {
    type Output = Wire<F>;

    fn mul(self, other: Wire<F>) -> Wire<F> {
        &self * &other
    }
}

impl<'a, 'b, F: IsField> std::ops::Mul<&'b Wire<F>> for &'a Wire<F> {
    type Output = Wire<F>;

    fn mul(self, other: &'b Wire<F>) -> Wire<F> {
        Wire {
            label: format!("({}*{})", self.label, other.label),
            value: self.value.clone() * other.value.clone(),
        }
    }
}

impl<F: IsPrimeField> std::ops::Sub<Wire<F>> for Wire<F> {
    type Output = Wire<F>;

    fn sub(self, other: Wire<F>) -> Wire<F> {
        &self - &other
    }
}

impl<'a, 'b, F: IsPrimeField> std::ops::Sub<&'b Wire<F>> for &'a Wire<F> {
    type Output = Wire<F>;

    fn sub(self, other: &'b Wire<F>) -> Wire<F> {
        self + &(-other)
    }
}

impl<F: IsPrimeField> std::ops::Neg for Wire<F> {
    type Output = Wire<F>;

    fn neg(self) -> Wire<F> {
        -&self
    }
}

impl<'a, F: IsPrimeField> std::ops::Neg for &'a Wire<F> {
    type Output = Wire<F>;

    fn neg(self) -> Wire<F> {
        &Wire::<F>::neg_one() * self
    }
}
