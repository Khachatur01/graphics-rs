pub trait Model {
    /**
     *  Returns the logical type identifier for this model, if applicable.
     *  This can be used to identify the specific kind of model (e.g., "coordinate_plane", "rectangle", "pointer", ...)
     *  for UI display or type-based operations.
     */
    fn logical_type_id(&self) -> Option<&'static str>;
}

#[derive(Debug)]
pub enum ModelConversionError {}

pub trait ModelInto<T> {
    fn try_into(&self) -> Result<T, ModelConversionError>;
}

pub trait ModelFrom<T> {
    fn try_from(input: &T) -> Result<impl Model, ModelConversionError>;
}
