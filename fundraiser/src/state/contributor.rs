use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

/// # Contributor State
///
/// > contributor: Pubkey
/// > amount: u64
pub struct Contributor(*const u8);

impl Contributor {
    #[allow(dead_code)]
    const LEN: usize = 40;

    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    #[allow(dead_code)]
    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Self::from_account_info_unchecked(account_info)
    }

    #[allow(dead_code)]
    pub fn contributor(&self) -> Pubkey {
        unsafe { *(self.0 as *const Pubkey) }
    }

    pub fn amount(&self) -> u64 {
        unsafe { *(self.0.add(32) as *const u64) }
    }
}
