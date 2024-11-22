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

const ID: [u8; 32] = decode_32_const("AMM9y52vqD1QgvX6oG5T1HX11VgCeQDnkEd66SmTSJCC");

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

    match AmmInstruction::try_from(discriminator)? {
        AmmInstruction::Initialize => initialize::process(accounts, data),
        AmmInstruction::Deposit => deposit::process(accounts, data),
        AmmInstruction::Withdraw => withdraw::process(accounts, data),
        AmmInstruction::Swap => todo!(),
        AmmInstruction::Lock => todo!(),
    }
}
