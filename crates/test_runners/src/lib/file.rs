use std::io::Write;

use borsh::BorshSerialize;
use eyre::Result;
use tempfile::NamedTempFile;

pub fn write_temporary(serializable: impl BorshSerialize) -> Result<NamedTempFile> {
    let borsh_serialized = serializable.try_to_vec()?;
    let mut file = NamedTempFile::new()?;
    file.write_all(&borsh_serialized)?;
    Ok(file)
}
