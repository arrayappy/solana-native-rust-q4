#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use mollusk_svm::Mollusk;
    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount},
        entrypoint::ProgramResult,
        instruction::{AccountMeta, Instruction},
        msg,
        pubkey::Pubkey,
    };

    #[test]
    fn withdraw() {
        // let program_id: Pubkey = pubkey!("VLTwrNB2Cv881QTojdfJHmkpwyCBLwCcSkp6R46kHMP");
        let program_id = Pubkey::new_from_array([
            0x07, 0x42, 0x45, 0x75, 0xa2, 0x8c, 0x39, 0xd3, 0xc4, 0xd6, 0x95, 0xee, 0x38, 0x83,
            0x80, 0x2e, 0x18, 0xad, 0xd4, 0x91, 0x73, 0x63, 0x0e, 0xff, 0x38, 0x33, 0xaa, 0x0e,
            0x89, 0xde, 0x81, 0x46,
        ]);

        let signer = Pubkey::new_unique();

        msg!("program_id: {:?}", program_id);
        let (vault, bump) =
            Pubkey::try_find_program_address(&[signer.as_ref()], &program_id).unwrap();

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[&1_000_000_000u64.to_le_bytes()[..], &[bump]].concat(),
            vec![
                AccountMeta::new(signer, true),
                AccountMeta::new(vault, false),
            ],
        );

        let mollusk = Mollusk::new(&program_id, "target/deploy/vault");

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (signer, AccountSharedData::new(0, 0, &Pubkey::default())),
                (
                    vault,
                    AccountSharedData::new(1_000_000_000u64, 0, &program_id),
                ),
            ],
        );

        assert_eq!(
            result.get_account(&signer).unwrap().lamports(),
            1_000_000_000
        );
        assert_eq!(result.get_account(&vault).unwrap().lamports(), 0);

        assert!(!result.program_result.is_err());
    }
}
