use lambdaworks_math::field::{
    element::FieldElement,
    traits::{IsField, IsPrimeField},
};

// TODO: we should add a `Context` ref field here that handles
// wire namings so that there are no collisions etc.

/// A circuit "wire" that holds a value and a label.
///
/// It has addition, multiplication, subtraction, and negation operations overloaded
/// so that the label is updated to indicate the operation w.r.t addition gates and
/// multiplication gates.
///
/// You can create constant wires (which are labeled by numbers) or named wires
/// with a label that you provide.
///
/// You can use `u64` values while creating constants, or even wires, which are implement
/// `Into` to become a field element in the corresponding field.
#[derive(Clone, Debug)]
pub struct Wire<F: IsField> {
    pub label: String,
    pub value: FieldElement<F>,
}

impl<F: IsPrimeField> Wire<F> {
    pub fn constant(value: impl Into<FieldElement<F>>) -> Self {
        let value = value.into();
        Self {
            label: value.representative().to_string(),
            value,
        }
    }

    pub fn new(value: impl Into<FieldElement<F>>, label: impl ToString) -> Self {
        Self {
            label: label.to_string(),
            value: value.into(),
        }
    }

    #[inline(always)]
    pub fn one() -> Self {
        Self::constant(1)
    }

    #[inline(always)]
    pub fn neg_one() -> Self {
        Self::constant(-FieldElement::<F>::one())
    }

    #[inline(always)]
    pub fn zero() -> Self {
        Self::constant(0)
    }
}

impl<F: IsField> PartialEq for Wire<F> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<F: IsField> Eq for &Wire<F> {}

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

impl<F: IsPrimeField> From<u64> for Wire<F> {
    fn from(value: u64) -> Self {
        Wire::constant(value)
    }
}
