use solana_program::{
  account_info::AccountInfo, entrypoint::ProgramResult, program::invoke,
  program_error::ProgramError, system_instruction::transfer,
};

/// Deposit funds into vault with deterministic address derived from Signer's pubkey
pub fn process(accounts: &[AccountInfo<'_>], amount: u64) -> ProgramResult {
  let [signer, vault, _system_program] = accounts else {
      return Err(ProgramError::NotEnoughAccountKeys);
  };

  invoke(&transfer(signer.key, vault.key, amount), accounts)
}