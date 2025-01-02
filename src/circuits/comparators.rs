use lambdaworks_math::field::{element::FieldElement, traits::IsPrimeField};

use super::Wire;

pub fn is_zero<F: IsPrimeField>(a: &Wire<F>) -> Wire<F> {
    // get inv as a hint
    let inv = if a.value == FieldElement::<F>::zero() {
        Wire::<F>::zero()
    } else {
        Wire::<F>::new(a.value.inv().unwrap(), format!("{}_neg", a.label))
    };

    let out = Wire::<F>::one() - (a * &inv);
    assert_eq!(a * &out, Wire::<F>::zero(), "in * out != 0");

    out
}

pub fn is_equal<F: IsPrimeField>(a: &Wire<F>, b: &Wire<F>) -> Wire<F> {
    is_zero(&(a - b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambdaworks_math::field::test_fields::u64_test_field::U64Field;

    type F = U64Field<17>;

    #[test]
    fn test_is_zero() {
        let one = Wire::<F>::one();
        let zero = Wire::<F>::zero();

        assert_eq!(is_zero(&zero), one);
        assert_eq!(is_zero(&one), zero);
    }

    #[test]
    fn test_is_equal() {
        let one = Wire::<F>::one();
        let zero = Wire::<F>::zero();
        let a = Wire::<F>::new(4, "a".to_string());
        let b = Wire::<F>::new(5, "b".to_string());

        assert_eq!(is_equal(&a, &b), zero);
        assert_eq!(is_equal(&a, &a), one);
        assert_eq!(is_equal(&b, &b), one);
        assert_eq!(is_equal(&b, &a), zero);
    }
}
