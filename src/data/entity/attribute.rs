use binary::datatypes::{Bool, F32};
use binary_derive::Binary;
use byteorder::LE;

use crate::data::{Slice, VarString};

#[derive(Default, Debug, Binary)]
pub struct Attribute {
    pub min: F32<LE>,
    pub max: F32<LE>,
    pub current: F32<LE>,
    pub default: F32<LE>,
    pub id: VarString,
    pub modifiers: Slice<AttributeModifier>,
}

#[derive(Default, Debug, Binary)]
pub struct AttributeModifier {
    pub id: VarString,
    pub name: VarString,
    pub amound: F32<LE>,
    pub operation: AttributeOperation,
    pub operand: AttributeTargetOperand,
    pub serializable: Bool,
}

#[derive(Default, Debug, Binary)]
#[data(datatype = "I32")]
pub enum AttributeOperation {
    Add,
    MultiplyBase,
    MultiplyTotal,
    Cap,
    #[default]
    Invalid,
}

#[derive(Default, Debug, Binary)]
#[data(datatype = "I32")]
pub enum AttributeTargetOperand {
    Min,
    Max,
    Current,
    #[default]
    Invalid,
}
