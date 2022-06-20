// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{verify_record_commitment, StateLeafValues, StateValues};
use leo_ast::Input as AstInput;
use leo_errors::{Result, SnarkVMError, StateError};

use snarkvm_algorithms::{
    commitment_tree::CommitmentMerklePath,
    traits::{CommitmentScheme, CRH},
};
use snarkvm_dpc::{
    testnet1::{instantiated::Components, parameters::SystemParameters},
    DPCComponents,
};
use snarkvm_utilities::{bytes::ToBytes, to_bytes_le, FromBytes};

use std::convert::TryFrom;

/// Returns `true` if the path to the local data commitment leaf is a valid path in the record
/// commitment Merkle tree.
pub fn verify_local_data_commitment(dpc: &SystemParameters<Components>, ast_input: &AstInput) -> Result<bool> {
    // Verify record commitment.
    let typed_record = ast_input.get_record();
    let dpc_record_values = verify_record_commitment(dpc, typed_record)?;
    let record_commitment: Vec<u8> = dpc_record_values.commitment;
    let record_serial_number: Vec<u8> = dpc_record_values.serial_number;

    // Parse typed state values.
    let typed_state = ast_input.get_state();
    let state_values = StateValues::try_from(typed_state)?;
    let leaf_index: u32 = state_values.leaf_index;
    let root: Vec<u8> = state_values.root;

    // parse typed state leaf values.
    let typed_state_leaf = ast_input.get_state_leaf();
    let state_leaf_values = StateLeafValues::try_from(typed_state_leaf)?;
    let path: Vec<u8> = state_leaf_values.path;
    let memo: Vec<u8> = state_leaf_values.memo;
    let network_id: u8 = state_leaf_values.network_id;
    let leaf_randomness: Vec<u8> = state_leaf_values.leaf_randomness;

    // Select local data commitment input bytes.
    let is_death = leaf_index < (Components::NUM_INPUT_RECORDS as u32);
    let input_bytes = if is_death {
        to_bytes_le![record_serial_number, record_commitment, memo, network_id].map_err(StateError::state_io_error)?
    } else {
        to_bytes_le![record_commitment, memo, network_id].map_err(StateError::state_io_error)?
    };

    // Construct local data commitment leaf.
    let local_data_leaf_randomness =
        <<Components as DPCComponents>::LocalDataCommitment as CommitmentScheme>::Randomness::read_le(
            &leaf_randomness[..],
        )
        .map_err(StateError::state_io_error)?;
    let local_data_commitment_leaf = <Components as DPCComponents>::LocalDataCommitment::commit(
        &dpc.local_data_commitment,
        &input_bytes,
        &local_data_leaf_randomness,
    )
    .map_err(|_| SnarkVMError::default())?;

    // Construct record commitment merkle path.
    let local_data_merkle_path = CommitmentMerklePath::<
        <Components as DPCComponents>::LocalDataCommitment,
        <Components as DPCComponents>::LocalDataCRH,
    >::read_le(&path[..])
    .map_err(StateError::state_io_error)?;

    // Check record commitment merkle path is valid for the given local data commitment root.
    let local_data_commitment_root = <<Components as DPCComponents>::LocalDataCRH as CRH>::Output::read_le(&root[..])
        .map_err(StateError::state_io_error)?;
    let result = local_data_merkle_path
        .verify(
            &dpc.local_data_crh,
            &local_data_commitment_root,
            &local_data_commitment_leaf,
        )
        .map_err(|_| SnarkVMError::default())?;

    Ok(result)
}
