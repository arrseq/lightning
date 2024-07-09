use strum_macros::FromRepr;
use instruction::operand;
use instruction::operand::{GetConfiguration, GetCodeConfiguration};
use utility::ToCode;

#[derive(Debug, Clone, Copy, FromRepr)]
#[repr(u16)]
pub enum Code {
    Add,
    CarryingAdd,
    Subtract,
    BorrowingSubtract,
    Multiply, 
    Divide,

    Copy,

    AppendStack,
    AppendStackRegisters,
    DetachStack,
    DetachStackRegisters,

    LogicalAnd,
    LogicalOr,
    LogicalNot,
    LogicalXor,

    Increment,
    Decrement,

    JumpIfZero,
    JumpIfOverflow,
    JumpIfRegrouping,
    JumpIfNegative
}

impl GetCodeConfiguration for Code {
    fn get_code_configuration(&self) -> Option<operand::ConfigurationCode> {
        Some(match self {
            Self::Add
            | Self::CarryingAdd
            | Self::Subtract
            | Self::BorrowingSubtract
            | Self::Multiply
            | Self::Divide
            | Self::Copy => operand::ConfigurationCode::Dual,
            Self::AppendStack
            | Self::DetachStack
            | Self::LogicalAnd
            | Self::LogicalOr
            | Self::LogicalNot
            | Self::LogicalXor
            | Self::Increment
            | Self::Decrement
            | Self::JumpIfZero
            | Self::JumpIfOverflow
            | Self::JumpIfRegrouping
            | Self::JumpIfNegative => operand::ConfigurationCode::Dynamic,
            Self::AppendStackRegisters
            | Self::DetachStackRegisters => return None
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Basic {
    Add(operand::SizedDual),
    CarryingAdd(operand::SizedDual),
    Subtract(operand::SizedDual),
    BorrowingSubtract(operand::SizedDual),
    Multiply(operand::SizedDual),
    Divide(operand::SizedDual),
    
    /// Copy the value between the operands corresponding to the result destination of the data.
    Copy(operand::SizedDual),
    
    /// Append and item from the operand onto the stack.
    AppendStack(operand::dynamic::SizedDynamic),
    /// Append all public registers to the stack. The stack pointer register will be the one before calling this 
    /// operation.
    AppendStackRegisters,
    /// Remove an item from the stack and store it in the operand.
    DetachStack(operand::dynamic::SizedDynamic),
    /// Copy the public registers from the stack and back into the public registers while removing them from the stack. 
    /// The stack pointer is not loaded back into the stack pointer register and only removed from the stack.
    DetachStackRegisters,
    
    LogicalAnd(operand::dynamic::SizedDynamic),
    LogicalOr(operand::dynamic::SizedDynamic),
    LogicalNot(operand::dynamic::SizedDynamic),
    LogicalXor(operand::dynamic::SizedDynamic),
    
    Increment(operand::dynamic::SizedDynamic),
    Decrement(operand::dynamic::SizedDynamic),
    
    JumpIfZero(operand::dynamic::SizedDynamic),
    JumpIfOverflow(operand::dynamic::SizedDynamic),
    JumpIfRegrouping(operand::dynamic::SizedDynamic),
    JumpIfNegative(operand::dynamic::SizedDynamic)
}

impl GetConfiguration for Basic {
    fn get_configuration(&self) -> Option<operand::Configuration> {
        Some(match self {
            Self::Add(x) 
                | Self::CarryingAdd(x)
                | Self::Subtract(x)
                | Self::BorrowingSubtract(x)
                | Self::Multiply(x)
                | Self::Divide(x)
                | Self::Copy(x) => operand::Configuration::Dual(*x),
            Self::AppendStack(x)
                | Self::DetachStack(x)
                | Self::LogicalAnd(x)
                | Self::LogicalOr(x)
                | Self::LogicalNot(x)
                | Self::LogicalXor(x)
                | Self::Increment(x)
                | Self::Decrement(x)
                | Self::JumpIfZero(x)
                | Self::JumpIfOverflow(x)
                | Self::JumpIfRegrouping(x)
                | Self::JumpIfNegative(x) => operand::Configuration::Dynamic(*x),
            Self::AppendStackRegisters 
                | Self::DetachStackRegisters => return None 
        })
    }
}

impl ToCode for Basic {
    type Code = u16;

    fn to_code(&self) -> Self::Code {
        (match self {
            Self::Add(_) => Code::Add,
            Basic::CarryingAdd(_) => Code::CarryingAdd,
            Basic::Subtract(_) => Code::Subtract,
            Basic::BorrowingSubtract(_) => Code::BorrowingSubtract,
            Basic::Multiply(_) => Code::Multiply,
            Basic::Divide(_) => Code::Divide,
            Basic::Copy(_) => Code::Copy,
            Basic::AppendStack(_) => Code::AppendStack,
            Basic::AppendStackRegisters => Code::AppendStackRegisters,
            Basic::DetachStack(_) => Code::DetachStack,
            Basic::DetachStackRegisters => Code::DetachStackRegisters,
            Basic::LogicalAnd(_) => Code::LogicalAnd,
            Basic::LogicalOr(_) => Code::LogicalOr,
            Basic::LogicalNot(_) => Code::LogicalNot,
            Basic::LogicalXor(_) => Code::LogicalXor,
            Basic::Increment(_) => Code::Increment,
            Basic::Decrement(_) => Code::Decrement,
            Basic::JumpIfZero(_) => Code::JumpIfZero,
            Basic::JumpIfOverflow(_) => Code::JumpIfOverflow,
            Basic::JumpIfRegrouping(_) => Code::JumpIfRegrouping,
            Basic::JumpIfNegative(_) => Code::JumpIfNegative
        }) as Self::Code
    }
}