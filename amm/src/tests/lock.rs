use mollusk_svm::{result::Check, Mollusk};
use solana_sdk::{
    account::{AccountSharedData, WritableAccount},
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use crate::state::Config;

#[test]
pub fn lock() {
    let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
        "AMM9y52vqD1QgvX6oG5T1HX11VgCeQDnkEd66SmTSJCC",
    ));

    let mollusk = Mollusk::new(&program_id, "target/deploy/amm");

    let authority = Pubkey::new_unique();
    let config = Pubkey::new_unique();

    let data = [4];

    let mut config_account = AccountSharedData::new(
        mollusk.sysvars.rent.minimum_balance(Config::LEN),
        Config::LEN,
        &program_id,
    );

    let mut config_data = [0u8; Config::LEN];
    config_data[0] = 0;
    config_data[1..33].copy_from_slice(&authority.to_bytes());
    config_data[33..65].copy_from_slice(&Pubkey::default().to_bytes());
    config_data[65..97].copy_from_slice(&Pubkey::default().to_bytes());
    config_data[97..129].copy_from_slice(&Pubkey::default().to_bytes());
    config_data[129..161].copy_from_slice(&Pubkey::default().to_bytes());
    config_data[161..193].copy_from_slice(&Pubkey::default().to_bytes());
    config_data[193..195].copy_from_slice(&1_000u16.to_le_bytes());
    config_data[195] = u8::MAX; 

    config_account.set_data_from_slice(&config_data);

    let instruction = Instruction::new_with_bytes(
        program_id,
        &data,
        vec![
            AccountMeta::new(authority, true),
            AccountMeta::new(config, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (authority, AccountSharedData::new(1_000_000_000u64, 0, &Pubkey::default())),
            (config, config_account),
        ],
        &[Check::success()]
    );
}