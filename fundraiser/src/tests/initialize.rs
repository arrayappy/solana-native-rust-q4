use mollusk_svm::{program, result::Check, Mollusk};
use solana_sdk::{
    account::{AccountSharedData, WritableAccount},
    instruction::{AccountMeta, Instruction},
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
};
use crate::state::Fundraiser;

#[test]
pub fn initialize() {
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

    let fundraiser = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let (vault, bump) = Pubkey::find_program_address(&[&fundraiser.to_bytes()], &program_id);

    let data = [
        vec![0],
        vec![bump],
        Pubkey::new_unique().to_bytes().to_vec(),
        i64::MAX.to_le_bytes().to_vec(),
        1_000_000u64.to_le_bytes().to_vec(),
    ]
    .concat();

    let mut mint_account = AccountSharedData::new(
        mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN),
        spl_token::state::Mint::LEN,
        &token_program,
    );
    solana_sdk::program_pack::Pack::pack(
        spl_token::state::Mint {
            mint_authority: COption::None,
            supply: 1_000_000,
            decimals: 6,
            is_initialized: true,
            freeze_authority: COption::None,
        },
        mint_account.data_as_mut_slice(),
    )
    .unwrap();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &data,
        vec![
            AccountMeta::new(fundraiser, true),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(token_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (
                fundraiser,
                AccountSharedData::new(
                    mollusk
                        .sysvars
                        .rent
                        .minimum_balance(Fundraiser::LEN),
                    Fundraiser::LEN,
                    &program_id,
                ),
            ),
            (mint, mint_account),
            (
                vault,
                AccountSharedData::new(
                    mollusk
                        .sysvars
                        .rent
                        .minimum_balance(spl_token::state::Account::LEN),
                    spl_token::state::Account::LEN,
                    &spl_token::ID,
                ),
            ),
            (token_program, token_program_account),
        ],
        &[Check::success()],
    );
} 