#![allow(dead_code)]

use macros::compare;
use paste::paste;

use crate::{asm::Mask, instruction, prelude::*, ParseError};

instruction!(
    size u32; NoTable contains
    VFMX32 : {
        sm      as u8   : u8            : 0 -> 3,
        m       as u8   : u8            : 5 -> 5,
        op      as u8   : u8            : 6 -> 6,
        n       as u8   : u8            : 7 -> 7,
        sd      as u8   : u8            : 12 -> 15,
        sn      as u8   : u8            : 16 -> 19,
        d       as u8   : u8            : 22 -> 22
    },
    VFMX64 : {
        sm      as u8   : u8            : 0 -> 3,
        m       as u8   : u8            : 5 -> 5,
        op      as u8   : u8            : 6 -> 6,
        n       as u8   : u8            : 7 -> 7,
        sd      as u8   : u8            : 12 -> 15,
        sn      as u8   : u8            : 16 -> 19,
        d       as u8   : u8            : 22 -> 22
    },
);
impl Parse for NoTable {
    type Target = Self;

    fn parse<T: Stream>(iter: &mut T) -> Result<Self::Target, ParseError>
    where
        Self: Sized,
    {
        let word: u32 = iter.peek::<1>().ok_or(ParseError::IncompleteProgram)?;

        if compare!(word == 1110 | 11101 | x | 10 | xxxx | xxxx | 101 | 0 | x | x | x | 0 | xxxx) {
            return Ok(NoTable::VFMX32(VFMX32::parse(iter)?));
        }
        if compare!(word == 1110 | 11101 | x | 10 | xxxx | xxxx | 101 | 1 | x | x | x | 0 | xxxx) {
            return Ok(NoTable::VFMX64(VFMX64::parse(iter)?));
        }
        Err(ParseError::Invalid32Bit("Not part of NoTable"))
    }
}

impl NoTable {
    pub fn encoding_specific_operations(self) -> Result<Operation, ParseError> {
        Ok(match self {
            Self::VFMX32(VFMX32 {
                sm,
                m,
                op,
                n,
                sd,
                sn,
                d,
            }) => {
                let sm = ((sm << 1) | m).try_into()?;
                let sn = ((sn << 1) | n).try_into()?;
                let sd = ((sd << 1) | d).try_into()?;
                let op = op != 0;
                Operation::VfmxF32(operation::VfmxF32 {
                    sd,
                    sn,
                    sm,
                    negate: op,
                })
            }
            Self::VFMX64(VFMX64 {
                sm,
                m,
                op,
                n,
                sd,
                sn,
                d,
            }) => {
                let dm = ((sm << 1) | m).try_into()?;
                let dn = ((sn << 1) | n).try_into()?;
                let dd = ((sd << 1) | d).try_into()?;
                let op = op != 0;
                Operation::VfmxF64(operation::VfmxF64 {
                    dd,
                    dn,
                    dm,
                    negate: op,
                })
            }
        })
    }
}
