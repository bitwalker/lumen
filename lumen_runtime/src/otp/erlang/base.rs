use std::convert::{TryFrom, TryInto};

use anyhow::*;

use liblumen_alloc::erts::term::prelude::*;

// 2-36
pub struct Base(u8);

impl Base {
    pub fn base(&self) -> u8 {
        self.0
    }

    pub fn radix(&self) -> u32 {
        self.0 as u32
    }

    const MIN: u8 = 2;
    const MAX: u8 = 36;
}

const CONTEXT: &str = "base must be an integer in 2-36";

impl TryFrom<Term> for Base {
    type Error = anyhow::Error;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        let base_u8: u8 = term.try_into().context(CONTEXT)?;

        if (Self::MIN <= base_u8) && (base_u8 <= Self::MAX) {
            Ok(Self(base_u8))
        } else {
            Err(TryIntoIntegerError::OutOfRange).context(CONTEXT)
        }
    }
}
