use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

use pinocchio_token::{
    instructions::{CloseAccount, Transfer},
    state::TokenAccount,
};

use crate::state::Escrow;

pub fn process(accounts: &[AccountInfo], bump: [u8; 1]) -> ProgramResult {
    let [maker, maker_ta_a, escrow, vault, authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(escrow.is_signer());

    // Unchecked because we'll be closing escrow, so it'll fail if owner is wrong
    let escrow_account = Escrow::from_account_info_unchecked(escrow);

    assert_eq!(&escrow_account.maker(), maker.key());

    // 1. Transfer tokens from vault to maker
    let seeds = [Seed::from(escrow.key().as_ref()), Seed::from(&bump)];
    let signers = [Signer::from(&seeds)];

    Transfer {
        from: vault,
        to: maker_ta_a,
        authority, // how authorty is working here?
        amount: TokenAccount::from_account_info_unchecked(vault).amount(),
    }
    .invoke_signed(&signers)?;

    // 2. Close vault
    CloseAccount {
        account: vault,
        destination: maker,
        authority,
    }
    .invoke_signed(&signers)?;

    // 3. Close escrow by draining lamports and setting data_len to 0
    unsafe {
        *maker.borrow_mut_lamports_unchecked() += *escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() = 0;

        escrow.assign(&Pubkey::default());

        // We are setting the account data_len to 0 - 8 bytes before the actual data
        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().sub(8) as *mut u64) = 0;
    }

    Ok(())
}
