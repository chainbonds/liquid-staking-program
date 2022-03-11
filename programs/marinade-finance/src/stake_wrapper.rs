use std::ops::Deref;
use std::io::Write;

use anchor_lang::prelude::ProgramError;
use anchor_lang::solana_program::stake::state::StakeState;
use anchor_lang::solana_program::pubkey::Pubkey;
pub use spl_token::ID;

#[derive(Debug, Clone)]
pub struct StakeWrapper {
    pub inner: StakeState,
}


impl anchor_lang::AccountDeserialize for StakeWrapper {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        Self::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self, ProgramError> {
        let result = Self {
            inner: bincode::deserialize(buf).map_err(|_| ProgramError::InvalidAccountData)?,
        };
        *buf = &buf[std::mem::size_of::<StakeState>()..];
        Ok(result)
    }
}


impl anchor_lang::AccountSerialize for StakeWrapper {
    fn try_serialize<W: Write>(&self, _write: &mut W) -> Result<(), ProgramError> {
        Ok(())
    }
}

impl Deref for StakeWrapper {
    type Target = StakeState;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}



impl anchor_lang::Owner for StakeWrapper {
    fn owner() -> Pubkey {
        ID
    }
}
