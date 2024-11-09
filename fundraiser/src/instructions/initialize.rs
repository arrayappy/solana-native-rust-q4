use pinocchio::{account_info::AccountInfo, instruction::{Seed, Signer}, program_error::ProgramError, pubkey::Pubkey, ProgramResult};
use pinocchio_token::instructions::InitilizeAccount3;

pub fn process(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [fundraiser, mint_to_raise, vault, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // fundraiser is a keypair account turned into a kinda PDA
    // Using keypair account for data storage saves CUs vs finding/creating PDAs while maintaining full control through ownership
    assert!(fundraiser.is_signer());

    let (bump, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;
    
    unsafe {
        let data_ptr = fundraiser.borrow_mut_data_unchecked().as_mut_ptr();

        *(data_ptr as *mut Pubkey) = *mint_to_raise.key(); // mint to raise
        *(data_ptr.add(32) as *mut Pubkey) = *(data.as_ptr() as *const Pubkey); // maker
        *(data_ptr.add(64) as *mut u64) = *(data.as_ptr().add(32) as *const u64); // amount to raise
        *(data_ptr.add(72) as *mut i64) = *(data.as_ptr().add(40) as *const i64); // time ending
    }

    let bump_bytes = bump.to_le_bytes();
    let seeds = [Seed::from(fundraiser.key().as_ref()), Seed::from(&bump_bytes)];
    let signer = [Signer::from(&seeds)];

    // Initialize token account owned by fundraiser PDA to store contributed tokens
    //
    // Before InitializeAccount3:
    // mint_to_raise (USDC Mint) ----→ vault (empty account)
    //
    // After InitializeAccount3:
    // mint_to_raise (USDC Mint) ----→ vault (Token Account initialized for USDC)
    //                                      └── owned by: fundraiser PDA
    InitilizeAccount3 {
        token: vault,
        owner: *fundraiser.key(),
        mint: mint_to_raise,
    }
    .invoke_signed(&signer)?;

    Ok(())
}