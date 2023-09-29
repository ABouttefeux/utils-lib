use syn::{Attribute, Expr};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Default)]
pub struct AttributeOption {
    expr: Option<Expr>,
}

impl AttributeOption {
    pub fn parse(vec: &[Attribute]) -> Self {
        todo!()
    }
}
