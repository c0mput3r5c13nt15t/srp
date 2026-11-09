/*
 *  SRP - Secure Reverse Proxy
 *  Copyright (C) 2026  SRP Team
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as
 *  published by the Free Software Foundation, either version 3 of the
 *  License, or (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use anyhow::{Context, Result};

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
