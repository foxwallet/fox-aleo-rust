// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

use snarkvm::prelude::{ConsensusVersion, InclusionVersion};
use snarkvm_console::network::varuna_version_from_consensus;
use super::*;

/// The consensus, Varuna, and inclusion versions active at a given block height.
#[derive(Clone, Copy, Debug)]
pub struct ProtocolVersions {
    pub consensus: ConsensusVersion,
    pub varuna: VarunaVersion,
    pub inclusion: InclusionVersion,
}

impl ProtocolVersions {
    /// Returns the versions active at the given block height, following the same rules as the VM.
    pub fn at_height<N: Network>(block_height: u32) -> Result<Self> {
        let consensus = N::CONSENSUS_VERSION(block_height)?;
        let varuna = varuna_version_from_consensus(consensus);
        let is_network_behind_upgrade_height = block_height < N::INCLUSION_UPGRADE_HEIGHT()?;
        let inclusion = match (ConsensusVersion::V1..=ConsensusVersion::V7).contains(&consensus)
            || is_network_behind_upgrade_height
        {
            true => InclusionVersion::V0,
            false => InclusionVersion::V1,
        };
        Ok(Self { consensus, varuna, inclusion })
    }

    /// Whether the stacks of directly imported programs are required for execution checks.
    pub fn include_direct_imports(&self) -> bool {
        self.consensus >= ConsensusVersion::V18
    }
}
