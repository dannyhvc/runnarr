use std::alloc::LayoutError;

#[derive(Debug)]
pub struct BaseError(pub String);

impl From<LayoutError> for BaseError {
    fn from(error_value: LayoutError) -> Self {
        BaseError(error_value.to_string())
    }
}
