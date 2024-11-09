use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    ProgramResult,
};
use pinocchio_token::{instructions::Transfer, state::TokenAccount};

use crate::state::Fundraiser;

pub fn process(accounts: &[AccountInfo], bump: [u8; 1]) -> ProgramResult {
    let [fundraiser, maker_ta, vault, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let fundraiser_account = Fundraiser::from_account_info(fundraiser);

    assert_eq!(
        fundraiser_account.maker(),
        TokenAccount::from_account_info_unchecked(maker_ta).authority()
    );

    let vault_account = TokenAccount::from_account_info_unchecked(vault);
    let amount = vault_account.amount();
    assert_eq!(vault_account.mint(), fundraiser_account.mint_to_raise());
    assert!(amount >= fundraiser_account.amount_to_raise());

    let seeds = [Seed::from(fundraiser.key().as_ref()), Seed::from(&bump)];
    let signers = [Signer::from(&seeds)];

    Transfer {
        from: vault,
        to: maker_ta,
        authority: vault,
        amount,
    }
    .invoke_signed(&signers)?;

    Ok(())
}
