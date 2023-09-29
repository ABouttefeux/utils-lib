use super::{option_enum::OptionEnum, option_struct::OptionStruct};

#[derive(Clone)]
pub enum NewOption {
    Struct(OptionStruct),
    Enum(OptionEnum),
}
