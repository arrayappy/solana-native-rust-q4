use pinocchio::program_error::ProgramError;

pub mod initialize;
pub mod contribute;
pub mod check;
pub mod refund;

#[derive(Debug, Clone, Copy)]
pub enum FundraiserInstruction {
    Initialize,
    Contribute,
    Check,
    Refund,
}

impl TryFrom<&u8> for FundraiserInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Initialize,
            1 => Self::Contribute,
            2 => Self::Check,
            3 => Self::Refund,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
