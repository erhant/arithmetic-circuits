use lambdaworks_math::field::traits::IsPrimeField;

use super::{
    bits::{assert_bit, not},
    Wire,
};

pub fn if_else<F: IsPrimeField>(c: &Wire<F>, t: &Wire<F>, f: &Wire<F>) -> Wire<F> {
    assert_bit(c);
    (t * c) + (f * &not(c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambdaworks_math::field::test_fields::u64_test_field::U64Field;

    type F = U64Field<17>;

    #[test]
    fn test_if_else() {
        let one = Wire::<F>::one();
        let zero = Wire::<F>::zero();

        let t = Wire::<F>::from(4);
        let f = Wire::<F>::from(8);

        assert_eq!(if_else(&zero, &t, &f), f);
        assert_eq!(if_else(&one, &t, &f), t);
    }
}
