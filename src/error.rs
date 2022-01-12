use solana_program::program_error::ProgramError;

pub enum AmmError {
    AmmError,
    InvalidStatus,
    InvalidFee,
}

impl Into<u32> for AmmError {
    fn into(self) -> u32 {
        match self {
            AmmError::AmmError => 0,
            AmmError::InvalidStatus => 1,
            AmmError::InvalidFee => 2,
        }
    }
}

impl Into<ProgramError> for AmmError {
    fn into(self) -> ProgramError {
        ProgramError::Custom(self.into())
    }
}