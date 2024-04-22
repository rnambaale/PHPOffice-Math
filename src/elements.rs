use self::{fraction::Fraction, identifier::Identifier, operator::Operator, superscript::Superscript};

pub trait CommonElementInterface {
    fn get_value(&self) -> &str;
}

pub mod fraction;
pub mod identifier;
pub mod numeric;
pub mod operator;
pub mod row;
pub mod superscript;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CommonElement {
    Identifier(Identifier),
    Operator(Operator),
    Superscript(Superscript),
    Fraction(Fraction),
}
