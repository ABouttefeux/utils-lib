use macro_utils::field::FieldInformation;

use super::attribute::AttributeOption;

#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct FieldOption {
    field: FieldInformation,
    attribute_option: AttributeOption,
}
