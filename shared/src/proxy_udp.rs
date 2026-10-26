use anyhow::{Result, Context};

pub const UDP_BUFFER_SIZE_BYTES: usize = 65_507;

pub fn parse_quic_data(data: &[u8]) -> Result<(u64, &[u8])> {
    let id_bytes: [u8; 8] = data
        .get(..8)
        .context("QUIC data too short to contain ID")?
        .try_into()?;

    let id = u64::from_be_bytes(id_bytes);
    let payload = &data[8..];

    Ok((id, payload))
}
