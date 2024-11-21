use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

/// State
///
/// status: u8 (1 byte)
/// update_authority: Pubkey (32 bytes)
/// mint_x: Pubkey (32 bytes)
/// mint_y: Pubkey (32 bytes)
/// mint_lp: Pubkey (32 bytes)
/// vault_x: Pubkey (32 bytes)
/// vault_y: Pubkey (32 bytes)
/// fee: u16 (2 bytes)
/// authority_bump: u8 (1 byte)
///

// Raw pointer to the account data
pub struct Config(*const u8);

impl Config {
    pub const LEN: usize = 1 + 32 + 32 + 32 + 32 + 32 + 32 + 2 + 1;

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Self::from_account_info_unchecked(account_info)
    }

    pub fn get_status(&self) -> u8 {
        unsafe { *(self.0 as *const u8) }
    }

    pub fn update_authority(&self) -> Pubkey {
        unsafe { *(self.0.add(1) as *const [u8; 32]) }
    }

    #[allow(unused)]
    pub fn mint_x(&self) -> Pubkey {
        unsafe { *(self.0.add(33) as *const [u8; 32]) }
    }

    #[allow(unused)]
    pub fn mint_y(&self) -> Pubkey {
        unsafe { *(self.0.add(65) as *const [u8; 32]) }
    }

    pub fn mint_lp(&self) -> Pubkey {
        unsafe { *(self.0.add(97) as *const [u8; 32]) }
    }

    pub fn vault_x(&self) -> Pubkey {
        unsafe { *(self.0.add(129) as *const [u8; 32]) }
    }

    pub fn vault_y(&self) -> Pubkey {
        unsafe { *(self.0.add(161) as *const [u8; 32]) }
    }

    pub fn fee(&self) -> u16 {
        unsafe { *(self.0.add(193) as *const u16) }
    }

    pub fn authority_bump(&self) -> u8 {
        unsafe { *(self.0.add(195) as *const u8) }
    }
}
