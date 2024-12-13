use serde::{Serialize, Serializer};
use std::error::Error;

pub(crate) type ApiResult<V: Serialize = ()> = Result<V, ApiError>;

#[derive(Debug)]
pub(crate) struct ApiError(Box<dyn Error>);

impl Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.to_string().as_str())
    }
}

impl<E: Error + 'static> From<E> for ApiError{
    fn from(value: E) -> Self {
        Self(Box::new(value))
    }
}