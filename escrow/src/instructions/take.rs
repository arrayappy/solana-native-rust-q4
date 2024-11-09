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
    let [taker, taker_ta_a, taker_ta_b, maker_ta_b, escrow, vault, authority, _token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Unchecked because we'll be closing escrow, so it'll fail if owner is wrong
    let escrow_account = Escrow::from_account_info_unchecked(escrow);

    // Ensures maker is not being rugged
    assert_eq!(&escrow_account.maker_ta_b(), maker_ta_b.key());

    // Ensures vault matches mint_a
    assert_eq!(
        TokenAccount::from_account_info_unchecked(vault).mint(),
        escrow_account.mint_a()
    );

    // 1. Transfer tokens from taker_ta_b to maker_ta_b
    Transfer {
        from: taker_ta_b,
        to: maker_ta_b,
        authority: taker,
        amount: escrow_account.amount_b(),
    }
    .invoke()?;

    // 2. Transfer tokens from vault to taker_ta_a
    let seeds = [Seed::from(escrow.key().as_ref()), Seed::from(&bump)];
    let signers = [Signer::from(&seeds)];

    Transfer {
        from: vault,
        to: taker_ta_a,
        authority,
        amount: TokenAccount::from_account_info_unchecked(vault).amount(),
    }
    .invoke_signed(&signers)?;

    // 3. Close vault
    CloseAccount {
        account: vault,
        destination: taker,
        authority,
    }
    .invoke_signed(&signers)?;

    // 4. Close escrow by draining lamports and setting data_len to 0
    unsafe {
        *taker.borrow_mut_lamports_unchecked() += *escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() = 0;

        escrow.assign(&Pubkey::default());

        *(escrow.borrow_mut_data_unchecked().as_mut_ptr().sub(8) as *mut u64) = 0;
    }

    Ok(())
}
