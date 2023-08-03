// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @title Pallet Collection Manager Interface
/// @author Freeverse team
/// @notice This interface allows Solidity contracts to interact with pallet-living-assets
/// @custom:address 0x0000000000000000000000000000000000000402
interface LivingAssets {
    /// @notice Creates a new collection
    /// @dev Call this function to create a new collection
    /// @return collection_id The unique ID of the newly created collection
    function createCollection() external returns (uint64);

    /// @notice Retrieves the owner of a specific collection
    /// @dev Call this function to get the owner of the specified collection
    /// @param collection_id The unique ID of the collection whose owner is to be retrieved
    /// @return The owner's address in bytes32 format
    function ownerOfCollection(
        uint64 collection_id
    ) external view returns (bytes32);
}