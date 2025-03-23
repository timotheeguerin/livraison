use msi_installer::tables::MsiDataBaseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("MsiDataBaseError: {0}")]
    MsiDataBase(#[from] MsiDataBaseError),
}
