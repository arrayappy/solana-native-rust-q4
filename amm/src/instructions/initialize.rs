use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

/// # Initialize Instruction Data
///
/// Seed: u16
/// Authority: Flag<Pubkey>
/// MintX: Pubkey
/// MintY: Pubkey
/// MintLP: Pubkey
/// VaultX: Pubkey
/// VaultY: Pubkey
/// Fee: u16
/// AuthorityBump: u8
/// 

pub fn process(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [config] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(config.is_signer());

    // copy data to config
    unsafe {
        *(config.borrow_mut_data_unchecked().as_ptr() as *mut &[u8]) = data;
    };

    Ok(())
}
