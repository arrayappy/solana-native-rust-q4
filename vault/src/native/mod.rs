mod instructions;
use instructions::*;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey, pubkey::Pubkey,
};

const ID: Pubkey = pubkey!("VLTwrNB2Cv881QTojdfJHmkpwyCBLwCcSkp6R46kHMP");

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    let amount = data
        .get(..8) // u64 requires 8 bytes
        .and_then(|bytes| bytes.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    match VaultInstructions::try_from(discriminator)? {
        VaultInstructions::Deposit => self::instructions::deposit::process(accounts, amount),
        VaultInstructions::Withdraw => self::instructions::withdraw::process(accounts, amount),
    }
}