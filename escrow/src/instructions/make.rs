use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    ProgramResult,
};

use crate::state::Escrow;

pub fn process(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [maker, escrow, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Escrow is a keypair account turned into a kinda PDA account upon creation
    // https://www.rareskills.io/post/solana-pda
    assert!(escrow.is_signer());

    // Not required - worst case maker loses tokens
    // assert!(maker.is_signer());

    let escrow_account = Escrow::from_account_info_unchecked(escrow);

    escrow_account.set_maker(*maker.key());

    escrow_account.set_remaining(&data);

    Ok(())
}
