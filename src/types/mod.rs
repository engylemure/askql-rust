pub mod base;
pub use base::*;
use std::borrow::Cow;

impl Type for bool {
    fn type_name() -> Cow<'static, str> {
        Cow::from("boolean")
    }
    fn validate<T>(_value: T) -> bool
    where
        T: Type,
    {
        T::type_name() == Self::type_name()
    }
}

impl Type for u32 {
    fn type_name() -> Cow<'static, str> {
        Cow::from("int")
    }
    fn validate<T>(_value: T) -> bool
    where
        T: Type,
    {
        T::type_name() == Self::type_name()
    }
}

impl Type for u64 {
    fn type_name() -> Cow<'static, str> {
        Cow::from("int")
    }
    fn validate<T>(_value: T) -> bool
    where
        T: Type,
    {
        T::type_name() == Self::type_name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boolean_type_name() {
        assert_eq!(Cow::from("boolean"), bool::type_name());
    }
    #[test]
    fn validate_boolean() {
        assert!(bool::validate(true));
        assert!(bool::validate(false));
        assert!(!bool::validate(0 as u32));
        assert!(!bool::validate(0 as u64));
    }

    #[test]
    fn validate_int() {
        assert!(u32::validate(0 as u64));
        assert!(u32::validate(0 as u32));
    }
}
