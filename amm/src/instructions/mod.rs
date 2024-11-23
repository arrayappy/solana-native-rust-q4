use pinocchio::program_error::ProgramError;

pub mod initialize;
pub mod deposit;
pub mod withdraw;
pub mod swap;
// pub mod lock;

#[derive(Debug, Clone, Copy)]
pub enum AmmInstruction {
    Initialize,
    Deposit,
    Withdraw,
    Swap,
    Lock,
}

impl TryFrom<&u8> for AmmInstruction {
    type Error = ProgramError;

    fn try_from(discriminator: &u8) -> Result<Self, Self::Error> {
        Ok(match discriminator {
            0 => Self::Initialize,
            1 => Self::Deposit,
            2 => Self::Withdraw,
            3 => Self::Swap,
            // 4 => Self::Lock,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
