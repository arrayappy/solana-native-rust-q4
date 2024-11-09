use pinocchio::program_error::ProgramError;

pub mod make;
pub mod refund;
pub mod take;

#[derive(Debug, Clone, Copy)]
pub enum EscrowInstruction {
    Make,
    Take,
    Refund,
}

impl TryFrom<&u8> for EscrowInstruction {
    type Error = ProgramError;

    fn try_from(discriminator: &u8) -> Result<Self, Self::Error> {
        Ok(match discriminator {
            0 => Self::Make,
            1 => Self::Take,
            2 => Self::Refund,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
