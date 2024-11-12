use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

/// State
///
/// maker (32 bytes)
/// maker_ta_b (32 bytes)
/// mint_a (32 bytes)
/// mint_b (32 bytes) - Not required as we have maker_ta_b, for RPC lookup
/// amount_b (8 bytes)
///

// Raw pointer to the account data
pub struct Escrow(*const u8);

impl Escrow {
    pub const LEN: usize = 136;

    // Owner check not required for writes and CPIs (fails if wrong), but required for reads
    #[inline(always)]
    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> Self {
        unsafe { Self(account_info.borrow_data_unchecked().as_ptr()) }
    }

    #[allow(unused)]
    #[inline(always)]
    pub fn from_account_info(account_info: &AccountInfo) -> Self {
        assert_eq!(account_info.data_len(), Self::LEN);
        assert_eq!(account_info.owner(), &crate::ID);
        Self::from_account_info_unchecked(account_info)
    }

    #[inline(always)]
    pub fn set_maker(&self, key: Pubkey) {
        // first 32 bytes
        unsafe { *(self.0 as *mut Pubkey) = key } //? using &key vs key -- key vs *key (in calling function)
    }

    pub fn set_remaining(&self, data: &[u8]) {
        unsafe {
            // 32 bytes to 136 bytes
            *(self.0.add(32) as *mut [u8; 104]) = *(data.as_ptr() as *const [u8; 104])
            //? accessing like this vs borrow_mut_data_unchecked
        }
    }

    #[inline(always)]
    pub fn maker(&self) -> Pubkey {
        unsafe { *(self.0 as *const Pubkey) }
    }

    #[inline(always)]
    pub fn maker_ta_b(&self) -> Pubkey {
        unsafe { *(self.0.add(32) as *const Pubkey) }
    }

    #[inline(always)]
    pub fn mint_a(&self) -> Pubkey {
        unsafe { *(self.0.add(64) as *const Pubkey) }
    }

    // Only used for RPC lookup
    // pub fn mint_b(&self) -> Pubkey {
    //   unsafe { *(self.0.add(96) as *const Pubkey) }
    // }

    #[inline(always)]
    pub fn amount_b(&self) -> u64 {
        unsafe { *(self.0.add(128) as *const u64) }
    }
}
