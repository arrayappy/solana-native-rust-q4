#[cfg(test)]
mod tests {
    use std::mem;

    use mollusk_svm::{program, Mollusk};

    use solana_sdk::{
        account::{AccountSharedData, WritableAccount},
        instruction::{AccountMeta, Instruction},
        program_option::COption,
        program_pack::Pack,
        pubkey::Pubkey,
    };
    use spl_token::state::AccountState;

    use crate::state::Escrow;

    #[test]
    fn make() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "FUNDkZTQdvxPnwSwvHTsgaTy7dppNsPynxSxtWqYLNuF",
        ));

        let mollusk = Mollusk::new(&program_id, "target/deploy/native_escrow");

        let maker = Pubkey::new_unique();
        let escrow = Pubkey::new_unique();
        let maker_ta_b = Pubkey::new_unique();
        let mint_a = Pubkey::new_unique();
        let mint_b = Pubkey::new_unique();

        let (system_program, system_program_account) = program::keyed_account_for_system_program();

        let data = [
            vec![0],
            maker_ta_b.to_bytes().to_vec(),
            mint_a.to_bytes().to_vec(),
            mint_b.to_bytes().to_vec(),
            1_000_000u64.to_le_bytes().to_vec(),
        ]
        .concat();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new(escrow, true), // It should be a signer because this account shouldn't exist yet
                AccountMeta::new_readonly(system_program, false),
            ],
        );

        let lamports = mollusk.sysvars.rent.minimum_balance(136);

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (
                    maker,
                    AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
                ),
                (escrow, AccountSharedData::new(lamports, 136, &program_id)),
                (system_program, system_program_account),
            ],
        );

        assert!(!result.program_result.is_err());
    }
}
