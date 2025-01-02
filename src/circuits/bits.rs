use lambdaworks_math::field::traits::IsPrimeField;

use super::Wire;

pub fn not<F: IsPrimeField>(b: &Wire<F>) -> Wire<F> {
    &Wire::<F>::one() - b
}

pub fn and<F: IsPrimeField>(a: &Wire<F>, b: &Wire<F>) -> Wire<F> {
    a * b
}

pub fn or<F: IsPrimeField>(a: &Wire<F>, b: &Wire<F>) -> Wire<F> {
    (a + b) - (a * b)
}

pub fn assert_bit<F: IsPrimeField>(b: &Wire<F>) {
    assert_eq!(&(b * b), b, "b is not a bit");
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambdaworks_math::field::test_fields::u64_test_field::U64Field;

    type F = U64Field<17>;

    #[test]
    fn test_not() {
        let one = Wire::<F>::one();
        let zero = Wire::<F>::zero();

        assert_eq!(not(&zero), one);
        assert_eq!(not(&one), zero);
    }

    #[test]
    fn test_and() {
        let one = Wire::<F>::one();
        let zero = Wire::<F>::zero();

        assert_eq!(and(&zero, &zero), zero);
        assert_eq!(and(&zero, &one), zero);
        assert_eq!(and(&one, &zero), zero);
        assert_eq!(and(&one, &one), one);
    }

    #[test]
    fn test_or() {
        let one = Wire::<F>::one();
        let zero = Wire::<F>::zero();

        assert_eq!(or(&zero, &zero), zero);
        assert_eq!(or(&zero, &one), one);
        assert_eq!(or(&one, &zero), one);
        assert_eq!(or(&one, &one), one);
    }
}
