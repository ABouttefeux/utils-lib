use syn::Variant;

use super::field::FieldOption;

#[derive(Clone)]
pub struct OptionEnum {
    variant: Variant,
    field_options: Vec<FieldOption>,
}
