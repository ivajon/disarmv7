//! Defines the statements available in armv7.

use crate::ParseError;

pub mod b16;
pub mod b32;

pub trait LocalTryInto<T> {
    fn local_try_into(self) -> Result<T, ParseError>;
}
/// Helper to mask fields from a type.
pub trait Mask {
    /// Masks out the bit field from START until END.
    fn mask<const START: usize, const END: usize>(&self) -> Self;
}

impl LocalTryInto<bool> for u8 {
    fn local_try_into(self) -> Result<bool, ParseError> {
        if self > 1 {
            return Err(ParseError::InvalidField(format!(
                "Invalid masking of bool {self}"
            )));
        }
        Ok(self != 0)
    }
}
impl LocalTryInto<bool> for u32 {
    fn local_try_into(self) -> Result<bool, ParseError> {
        if self > 1 {
            return Err(ParseError::InvalidField(format!(
                "Invalid masking of bool {self}"
            )));
        }
        Ok(self != 0)
    }
}
impl Mask for u8 {
    #[allow(clippy::cast_possible_truncation)]
    fn mask<const START: usize, const END: usize>(&self) -> Self {
        let intermediate = self >> START;
        let mask = (1 << (END - START + 1) as Self) - 1;

        let ret = intermediate & mask;
        assert!(ret <= mask);
        ret
    }
}
impl Mask for u16 {
    #[allow(clippy::cast_possible_truncation)]
    fn mask<const START: usize, const END: usize>(&self) -> Self {
        let intermediate = self >> START;
        let mask = (1 << (END - START + 1) as Self) - 1;

        let ret = intermediate & mask;
        assert!(ret <= mask);
        ret
    }
}

impl Mask for u32 {
    #[allow(clippy::cast_possible_truncation)]
    fn mask<const START: usize, const END: usize>(&self) -> Self {
        let intermediate = self >> START;
        let mask = (1 << (END - START + 1) as Self) - 1;

        let ret = intermediate & mask;
        assert!(ret <= mask);
        ret
    }
}
#[cfg(test)]
mod test {
    use super::Mask;

    #[test]
    fn test_mask_u16() {
        let num: u16 = 0b10011;
        let first_two = num.mask::<0, 1>();
        let other = num.mask::<1, 2>();
        assert_eq!(first_two, 0b11);
        assert_eq!(other, 0b01);
    }
    #[test]
    fn test_mask_u32() {
        let num: u32 = 0b10011;
        let first_two = num.mask::<0, 1>();
        let other = num.mask::<1, 2>();
        assert_eq!(first_two, 0b11);
        assert_eq!(other, 0b01);
    }
}
