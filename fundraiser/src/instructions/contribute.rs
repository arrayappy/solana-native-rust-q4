use pinocchio_token::{instructions::Transfer, state::TokenAccount};
use solana_nostd_sha256::hashv;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, sysvars::{clock::Clock, Sysvar}, ProgramResult};

use crate::{state::{Contributor, Fundraiser}, ID, PDA_MARKER};

pub fn process(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [contributor, fundraiser, contributor_account, contributor_ta, vault, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let (bump, data) = data
        .split_first()
        .ok_or(pinocchio::program_error::ProgramError::InvalidInstructionData)?;

    let pda = hashv(&[
        fundraiser.key().as_ref(),
        &[*bump],
        ID.as_ref(),
        PDA_MARKER,
    ]);

    assert_eq!(pda, vault.key().as_ref());

    let fundraiser_account = Fundraiser::from_account_info(fundraiser);
    assert_eq!(fundraiser_account.mint_to_raise(), TokenAccount::from_account_info_unchecked(vault).mint());

    let amount = unsafe { *(data.as_ptr() as *const u64) };

    unsafe {
      let data_ptr = contributor_account.borrow_mut_data_unchecked().as_mut_ptr();
      if contributor_account.data_len() != 0 {
        *(data_ptr.add(32) as *mut [u8; 8]) = (Contributor::from_account_info_unchecked(contributor_account).amount() + amount).to_le_bytes();
      } else {
        *(data_ptr as *mut Pubkey) = *contributor.key();
        *(data_ptr.add(32) as *mut [u8; 8]) = amount.to_le_bytes();
      }
    }

    let current_time = Clock::get()?.unix_timestamp;
    assert!(fundraiser_account.time_ending() > current_time);

    Transfer {
        from: contributor_ta,
        to: vault,
        authority: contributor,
        amount,
    }
    .invoke()?;

    Ok(())
}