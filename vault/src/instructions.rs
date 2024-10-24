use solana_program::program_error::ProgramError;
pub enum VaultInstructions {
    Deposit,
    Withdraw,
}

impl TryFrom<&u8> for VaultInstructions {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Deposit),
            1 => Ok(Self::Withdraw),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}