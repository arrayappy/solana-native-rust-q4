use mollusk_svm::{program, result::Check, Mollusk};
use solana_sdk::{
    account::{AccountSharedData, WritableAccount},
    instruction::{AccountMeta, Instruction},
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
};
use crate::state::{Contributor, Fundraiser};

#[test]
pub fn contribute() {
    let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
        "FUNDkZTQdvxPnwSwvHTsgaTy7dppNsPynxSxtWqYLNuF",
    ));

    let mut mollusk = Mollusk::new(&program_id, "target/deploy/fundraiser");

    mollusk.add_program(
        &spl_token::ID,
        "src/tests/spl_token-3.5.0",
        &mollusk_svm::program::loader_keys::LOADER_V3,
    );

    let (token_program, token_program_account) = (
        spl_token::ID,
        program::create_program_account_loader_v3(&spl_token::ID),
    );

    let contributor = Pubkey::new_unique();
    let fundraiser = Pubkey::new_unique();
    let contributor_account = Pubkey::new_unique();
    let contributor_ta = Pubkey::new_unique();
    let (vault, bump) = Pubkey::find_program_address(&[&fundraiser.to_bytes()], &program_id);

    let data = [
        vec![1],
        vec![bump],
        1_000u64.to_le_bytes().to_vec(),
    ]
    .concat();

    let mut vault_account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &spl_token::ID,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: Pubkey::default(),
            owner: vault,
            amount: 0,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        vault_account.data_as_mut_slice(),
    ).unwrap();

    let mut contributor_ta_account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN),
        spl_token::state::Account::LEN,
        &spl_token::ID,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Account {
            mint: Pubkey::default(),
            owner: contributor,
            amount: 1_000_000,
            delegate: COption::None,
            state: spl_token::state::AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        },
        contributor_ta_account.data_as_mut_slice(),
    ).unwrap();

    let mut fundraiser_account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(Fundraiser::LEN),
        Fundraiser::LEN,
        &program_id,
    );
    fundraiser_account.set_data_from_slice(
        &[
            Pubkey::default().to_bytes().to_vec(),
            Pubkey::default().to_bytes().to_vec(),
            i64::MAX.to_le_bytes().to_vec(),
            1_000_000u64.to_le_bytes().to_vec(),
        ]
        .concat(),
    );

    let instruction = Instruction::new_with_bytes(
        program_id,
        &data,
        vec![
            AccountMeta::new(contributor, true),
            AccountMeta::new(fundraiser, false),
            AccountMeta::new(contributor_account, false),
            AccountMeta::new(contributor_ta, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(token_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (contributor, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
            (fundraiser, fundraiser_account),
            (
                contributor_account, 
                AccountSharedData::new(
                    mollusk
                        .sysvars
                        .rent
                        .minimum_balance(Contributor::LEN),
                    Contributor::LEN,
                    &program_id,
                ),
            ),
            (contributor_ta, contributor_ta_account),
            (vault, vault_account),
            (token_program, token_program_account),
        ],
        &[Check::success()],
    );
} 