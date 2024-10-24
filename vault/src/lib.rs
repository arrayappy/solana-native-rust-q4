mod instructions;
use instructions::*;
mod tests;

mod deposit;
mod withdraw;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey, pubkey::Pubkey,
};

const ID: Pubkey = pubkey!("VLTwrNB2Cv881QTojdfJHmkpwyCBLwCcSkp6R46kHMP");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&crate::ID) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (discriminator, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    let amount = data
        .get(..8) // u64 requires 8 bytes
        .and_then(|bytes| bytes.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    match VaultInstructions::try_from(discriminator)? {
        VaultInstructions::Deposit => deposit::process(accounts, amount),
        VaultInstructions::Withdraw => withdraw::process(accounts, amount),
    }
}