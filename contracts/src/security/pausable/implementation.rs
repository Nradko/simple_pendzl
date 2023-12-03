// SPDX-License-Identifier: MIT

pub use super::{Pausable, PausableError, PausableInternal, PausableStorage, Paused, Unpaused};
use pendzl::traits::{AccountId, Storage};

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct Data {
    #[lazy]
    pub paused: bool,
}

impl PausableStorage for Data {
    fn paused(&self) -> bool {
        self.paused.get().unwrap_or(true)
    }

    fn set_paused(&mut self, pause: bool) {
        self.paused.set(&pause);
    }
}

pub trait PausableImpl: PausableInternal {
    fn paused_impl(&self) -> bool {
        self._paused()
    }
}

pub trait PausableInternalImpl: Storage<Data>
where
    Data: PausableStorage,
{
    fn _paused_impl(&self) -> bool {
        self.data().paused()
    }

    fn _pause_impl(&mut self) -> Result<(), PausableError> {
        self._ensure_not_paused_impl()?;
        self.data().set_paused(true);
        let account = Self::env().caller();
        Self::env().emit_event(Paused {
            account: Self::env().caller(),
        });
        Ok(())
    }

    fn _unpause_impl(&mut self) -> Result<(), PausableError> {
        self._ensure_paused_impl()?;
        self.data().set_paused(false);
        Self::env().emit_event(Unpaused {
            account: Self::env().caller(),
        });
        Ok(())
    }

    fn _ensure_paused_impl(&self) -> Result<(), PausableError> {
        if !self.data().paused.get_or_default() {
            return Err(From::from(PausableError::NotPaused));
        }

        Ok(())
    }

    fn _ensure_not_paused_impl(&self) -> Result<(), PausableError> {
        if self.data().paused.get_or_default() {
            return Err(From::from(PausableError::Paused));
        }

        Ok(())
    }
}
