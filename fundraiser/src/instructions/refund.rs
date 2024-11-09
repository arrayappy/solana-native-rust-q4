use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};
use pinocchio_token::{instructions::Transfer, state::TokenAccount};

use crate::state::{Contributor, Fundraiser};

pub fn process(accounts: &[AccountInfo], bump: [u8; 1]) -> ProgramResult {
    let [contributor, fundraiser, contributor_ta, vault, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(contributor.is_signer());

    let fundraiser_account = Fundraiser::from_account_info(fundraiser);

    assert!(fundraiser_account.time_ending() >= Clock::get()?.unix_timestamp);

    let vault_account = TokenAccount::from_account_info_unchecked(vault);
    assert!(fundraiser_account.amount_to_raise() > vault_account.amount());
    assert_eq!(fundraiser_account.mint_to_raise(), vault_account.mint());

    let seeds = [Seed::from(fundraiser.key().as_ref()), Seed::from(&bump)];
    let signers = [Signer::from(&seeds)];

    Transfer {
        from: vault,
        to: contributor_ta,
        authority: vault,
        amount: Contributor::from_account_info_unchecked(contributor).amount(),
    }
    .invoke_signed(&signers)?;

    Ok(())
}
