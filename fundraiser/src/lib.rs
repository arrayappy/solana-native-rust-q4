use five8_const::decode_32_const;

use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;
use pinocchio::{entrypoint, ProgramResult};

mod instructions;
use instructions::*;
mod state;

const ID: [u8; 32] = decode_32_const("FUNDkZTQdvxPnwSwvHTsgaTy7dppNsPynxSxtWqYLNuF");

pub const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

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

    match FundraiserInstruction::try_from(discriminator)? {
        FundraiserInstruction::Initialize => initialize::process(accounts, data)?,
        FundraiserInstruction::Contribute => contribute::process(accounts, data)?,
        FundraiserInstruction::Check => check::process(accounts, [data[0]])?,
        FundraiserInstruction::Refund => refund::process(accounts, [data[0]])?,
    }

    Ok(())
}
