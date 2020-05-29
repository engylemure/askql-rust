use std::borrow::Cow;
use std::sync::Arc;

pub trait Type {
    fn type_name() -> Cow<'static, str>;
    fn qualified_type_name() -> String {
        format!("{}!", Self::type_name())
    }
    fn validate<T>(_value: T) -> bool
    where
        T: Type,
    {
        true
    }
}

impl<T: Type + Send + Sync> Type for &T {
    fn type_name() -> Cow<'static, str> {
        T::type_name()
    }
    fn validate<I>(_value: I) -> bool
    where
        I: Type,
    {
        I::type_name() == T::type_name()
    }
}

impl<T: Type + Send + Sync> Type for Arc<T> {
    fn type_name() -> Cow<'static, str> {
        T::type_name()
    }
    fn validate<I>(_value: I) -> bool
    where
        I: Type,
    {
        I::type_name() == T::type_name()
    }
}

impl<T: Type + Send + Sync> Type for Option<T> {
    fn type_name() -> Cow<'static, str> {
        T::type_name()
    }
    fn validate<I>(_value: I) -> bool
    where
        I: Type,
    {
        I::type_name() == T::type_name()
    }
}
