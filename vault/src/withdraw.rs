use solana_program::{
  account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed,
  program_error::ProgramError, pubkey::Pubkey, system_instruction::transfer,
};

/// Withdraw funds from vault with deterministic address derived from Signer's pubkey
pub fn process(accounts: &[AccountInfo<'_>], amount: u64) -> ProgramResult {
  let [vault, signer, _system_program] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
  };

  let (pda, bump) = Pubkey::try_find_program_address(&[signer.key.as_ref()], &crate::ID)
      .ok_or(ProgramError::InvalidSeeds)?;

  assert_eq!(&pda, vault.key);

  invoke_signed(
      &transfer(
        vault.key,
        signer.key,
        amount
      ),
      accounts,
      &[&[signer.key.as_ref(), &[bump]]],
  )
}