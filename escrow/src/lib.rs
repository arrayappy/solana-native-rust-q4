use five8_const::decode_32_const;

use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

mod instructions;
use instructions::*;

pub mod state;

#[cfg(test)]
mod tests;

const ID: [u8; 32] = decode_32_const("ESCr1yhmVUrX8vRURmZRnGM4QugeFMHvY2ABUa2Cewrh");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id != &ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (discriminator, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match EscrowInstruction::try_from(discriminator)? {
        EscrowInstruction::Make => make::process(accounts, data),
        EscrowInstruction::Take => take::process(accounts, [data[0]]),
        EscrowInstruction::Refund => refund::process(accounts, [data[0]]),
    }
}
