use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

/// # Fundraiser State
///
/// > maker: Pubkey
/// > mint_to_raise: Pubkey
/// > amount_to_raise: u64
/// > time_ending: i64
pub struct Fundraiser(*const u8);

impl Fundraiser {
    const LEN: usize = 80;

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    #[inline(always)]
    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Self::from_account_info_unchecked(account_info)
    }

    pub fn mint_to_raise(&self) -> Pubkey {
        unsafe { *(self.0 as *const Pubkey) }
    }

    pub fn maker(&self) -> Pubkey {
        unsafe { *(self.0.add(32) as *const Pubkey) }
    }

    pub fn amount_to_raise(&self) -> u64 {
        unsafe { *(self.0.add(64) as *const u64) }
    }

    pub fn time_ending(&self) -> i64 {
        unsafe { *(self.0.add(72) as *const i64) }
    }
}
