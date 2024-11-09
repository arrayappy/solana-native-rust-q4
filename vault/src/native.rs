use solana_program::entrypoint;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, hash::hashv, program_error::ProgramError,
    pubkey::Pubkey,
};

// const ID: Pubkey = pubkey!("VLTwrNB2Cv881QTojdfJHmkpwyCBLwCcSkp6R46kHMP");
const ID: Pubkey = Pubkey::new_from_array([
    0x07, 0x42, 0x45, 0x75, 0xa2, 0x8c, 0x39, 0xd3, 0xc4, 0xd6, 0x95, 0xee, 0x38, 0x83, 0x80, 0x2e,
    0x18, 0xad, 0xd4, 0x91, 0x73, 0x63, 0x0e, 0xff, 0x38, 0x33, 0xaa, 0x0e, 0x89, 0xde, 0x81, 0x46,
]);

const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

entrypoint!(process_instruction);

/// # Withdraw
///
/// Handles withdrawing funds from a PDA that has previously had lamports deposited to it.
///
/// We don't need to "Deposit" instruction because of intent:
/// Vault get's created and deposited in an instruction of this transaction to avoid the
/// transfer CPI -> This works because if the vault actually doesn't have any lamports,
/// nobody will want to exchange it for the other token.
///
/// Note: every CPI costs 1000 CUs, so we should avoid it as much as possible.
///
/// The trade-off of saving more CUs (from CPIs) is getting more client-heavy approach.
/// In client-side, we `create` and `transfer` lamports to vault before calling this instruction.
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let [signer, vault] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(signer.is_signer);

    let lamports: u64 = u64::from_le_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]);
    let bump = data[8];
    let pda = hashv(&[signer.key.as_ref(), &[bump], ID.as_ref(), PDA_MARKER]);

    assert_eq!(pda.to_bytes(), vault.key.as_ref());

    **vault.try_borrow_mut_lamports()? -= lamports;
    **signer.try_borrow_mut_lamports()? += lamports;

    Ok(())
}
