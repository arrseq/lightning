use strum_macros::FromRepr;
use instruction::operand;
use instruction::operand::{Configuration, ConfigurationCode, GetConfiguration, GetCodeConfiguration, SizedDual};
use instruction::operation::basic::Basic;
use instruction::operation::floating::Floating;
use number::low::{LowNumber, LowSize};
use utility::ToCode;

pub mod basic;
pub mod floating;

#[derive(Default, Debug, Clone, Copy, FromRepr)]
pub enum Extension {
    #[default]
    Basic,
    Floating
}

#[derive(Debug, Clone, Copy)]
pub enum Code {
    Basic(basic::Code),
    Floating(floating::Code)
}

impl Code {
    pub fn from_extension_and_operation(extension: Extension, operation: u16) -> Option<Self> {
        Some(match extension {
            Extension::Basic => Code::Basic(basic::Code::from_repr(operation)?),
            Extension::Floating => Code::Floating(floating::Code::from_repr(operation)?)
        })
    }
}

impl GetCodeConfiguration for Code {
    fn get_code_configuration(&self) -> Option<ConfigurationCode> {
        match self {
            Self::Basic(basic) => basic.get_code_configuration(),
            Self::Floating(floating) => floating.get_code_configuration()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Basic(Basic),
    Floating(Floating)
}

impl From<Operation> for Extension {
    fn from(value: Operation) -> Self {
        match value {
            Operation::Basic(_) => Extension::Basic,
            Operation::Floating(_) => Extension::Floating
        }
    }
}

impl ToCode for Operation {
    type Code = u16;

    fn to_code(&self) -> Self::Code {
        match self {
            Self::Basic(x) => x.to_code(),
            Self::Floating(x) => x.to_code()
        }
    }
}

impl GetConfiguration for Operation {
    fn get_configuration(&self) -> Option<operand::Configuration> {
        match self {
            Self::Basic(x) => x.get_configuration(),
            Self::Floating(x) => x.get_configuration()
        }
    }
}

#[derive(Debug)]
pub enum OperationDecodeError {

}

impl Operation {
    /// Convert the operation to a code, then store it in a [LowNumber] type corresponding to what value [LowSize] is. This
    /// could result in the operation code losing data and start referring to a different operation. This behavior could
    /// be undefined.
    pub fn force_code_constrained(&self, size: &LowSize) -> LowNumber {
        let code = self.to_code();

        match size {
            LowSize::Byte => LowNumber::Byte(code as u8),
            LowSize::Word => LowNumber::Word(code)
        }
    }

    /// Convert the operation to a code and use the smallest data type that can represent that operation.
    pub fn to_smallest_code(&self) -> LowNumber {
        let code = self.to_code();

        if code > u8::MAX as u16 { return LowNumber::Word(code); }
        LowNumber::Byte(code as u8)
    }

    pub fn from_sized_dual(operation: Code, operands: SizedDual) -> Option<Self> {
        Some(match operation {
            Code::Basic(basic) => Self::Basic(Basic::from_sized_dual(basic, operands)?),
            Code::Floating(floating) => Self::Floating(Floating::from_sized_dual(floating, operands)?)
        })
    }
}