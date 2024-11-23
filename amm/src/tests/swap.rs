use mollusk_svm::{result::Check, Mollusk};
use solana_sdk::{
    account::{AccountSharedData, WritableAccount},
    instruction::{AccountMeta, Instruction},
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::AccountState;
use crate::state::Config;

#[test]
pub fn swap() {
    let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
        "AMM9y52vqD1QgvX6oG5T1HX11VgCeQDnkEd66SmTSJCC",
    ));

    let mut mollusk = Mollusk::new(&program_id, "target/deploy/amm");
    
    mollusk_token::token::add_program(&mut mollusk);
    let (token_program, token_program_account) = mollusk_token::token::keyed_account();

    let user = Pubkey::new_unique();
    let config = Pubkey::new_unique();
    let (authority, bump) = Pubkey::find_program_address(&[config.as_ref()], &program_id);
    let user_x = Pubkey::new_unique();
    let user_y = Pubkey::new_unique();
    let vault_from = Pubkey::new_unique();
    let vault_to = Pubkey::new_unique();

    let data = [
        vec![3],
        1_000_000u64.to_le_bytes().to_vec(),
        1_000u64.to_le_bytes().to_vec(),
        i64::MIN.to_le_bytes().to_vec(),
    ]
    .concat();

    let mut user_x_account = AccountSharedData::new(
        mollusk
        .sysvars
        .rent
        .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &spl_token::id(),
    );
    spl_token::state::Account::pack(
        spl_token::state::Account {
            mint: Pubkey::default(),
            owner: user,
            amount: 1_000_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_x_account.data_as_mut_slice(),
    ).unwrap();

    let mut user_y_account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &spl_token::id(),
    );
    spl_token::state::Account::pack(
        spl_token::state::Account {
            mint: Pubkey::default(),
            owner: user,
            amount: 0,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        user_y_account.data_as_mut_slice(),
    ).unwrap();

    let mut vault_from_account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &spl_token::id(),
    );
    spl_token::state::Account::pack(
        spl_token::state::Account {
            mint: Pubkey::default(),
            owner: authority,
            amount: 1_000_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_from_account.data_as_mut_slice(),
    ).unwrap();

    let mut vault_to_account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &spl_token::id(),
    );
    spl_token::state::Account::pack(
        spl_token::state::Account {
            mint: Pubkey::default(),
            owner: authority,
            amount: 1_000_000_000,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_to_account.data_as_mut_slice(),
    ).unwrap();

    let mut config_account = AccountSharedData::new(
        mollusk.sysvars.rent.minimum_balance(Config::LEN),
        Config::LEN,
        &program_id,
    );

    let mut config_data = [0u8; Config::LEN];
    config_data[0] = 0;
    config_data[1..33].copy_from_slice(&Pubkey::default().to_bytes());
    config_data[33..65].copy_from_slice(&Pubkey::default().to_bytes());
    config_data[65..97].copy_from_slice(&Pubkey::default().to_bytes());
    config_data[97..129].copy_from_slice(&Pubkey::default().to_bytes());
    config_data[129..161].copy_from_slice(&vault_from.to_bytes());
    config_data[161..193].copy_from_slice(&vault_to.to_bytes());
    config_data[193..195].copy_from_slice(&1_000u16.to_le_bytes());
    config_data[195] = bump; 

    config_account.set_data_from_slice(&config_data);

    let instruction = Instruction::new_with_bytes(
        program_id,
        &data,
        vec![
            AccountMeta::new(user, true),
            AccountMeta::new(authority, false),
            AccountMeta::new(user_x, false),
            AccountMeta::new(user_y, false),
            AccountMeta::new(vault_from, false),
            AccountMeta::new(vault_to, false),
            AccountMeta::new(config, false),
            AccountMeta::new(token_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (user, AccountSharedData::new(1_000_000_000u64, 0, &Pubkey::default())),
            (authority, AccountSharedData::new(1_000_000_000u64, 0, &Pubkey::default())),
            (user_x, user_x_account),
            (user_y, user_y_account),
            (vault_from, vault_from_account),
            (vault_to, vault_to_account),
            (config, config_account),
            (token_program, token_program_account),
        ],
        &[Check::success()]
    );
}