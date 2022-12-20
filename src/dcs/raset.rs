//! Module for the RASET address window instruction constructors

use crate::{instruction::Instruction, Error};

use super::DcsCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Raset {
    start_row: u16,
    end_row: u16,
}

impl Raset {
    ///
    /// Construct a new Raset range
    ///
    pub fn new(start_row: u16, end_row: u16) -> Self {
        Self { start_row, end_row }
    }
}

impl DcsCommand for Raset {
    fn instruction(&self) -> Instruction {
        Instruction::RASET
    }

    fn fill_params_buf(&self, buffer: &mut [u8]) -> Result<usize, Error> {
        buffer[0..2].copy_from_slice(&self.start_row.to_be_bytes());
        buffer[2..4].copy_from_slice(&self.end_row.to_be_bytes());

        Ok(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raset_fills_data_properly() -> Result<(), Error> {
        let raset = Raset::new(0, 320);

        let mut buffer = [0u8; 4];
        assert_eq!(raset.fill_params_buf(&mut buffer)?, 4);
        assert_eq!(buffer, [0, 0, 0x1, 0x40]);

        Ok(())
    }
}
