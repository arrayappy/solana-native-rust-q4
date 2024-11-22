use mollusk_svm::{result::Check, Mollusk};
use solana_sdk::{
    account::{AccountSharedData, WritableAccount},
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use crate::state::Config;

#[test]
pub fn initialize() {
    let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
        "AMM9y52vqD1QgvX6oG5T1HX11VgCeQDnkEd66SmTSJCC",
    ));

    let mollusk = Mollusk::new(&program_id, "target/deploy/amm");

    let config = Pubkey::new_unique();

    let data = [
        vec![0],
        vec![0],
        Pubkey::default().to_bytes().to_vec(),
        Pubkey::default().to_bytes().to_vec(),
        Pubkey::default().to_bytes().to_vec(),
        Pubkey::default().to_bytes().to_vec(),
        Pubkey::default().to_bytes().to_vec(),
        Pubkey::default().to_bytes().to_vec(),            
        u16::MAX.to_le_bytes().to_vec(),
        u8::MAX.to_le_bytes().to_vec(),
    ]
    .concat();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &data,
        vec![
            AccountMeta::new(config, true),
        ],
    );

    let lamports = mollusk.sysvars.rent.minimum_balance(Config::LEN);

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (config, AccountSharedData::new(lamports, Config::LEN, &program_id)),
        ],
        &[Check::success()]
    );
}