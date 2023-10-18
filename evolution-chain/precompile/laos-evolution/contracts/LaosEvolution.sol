// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @title Pallet Collection Manager Interface
/// @author LAOS Team
/// @notice This interface allows Solidity contracts to interact with pallet-living-assets
/// @custom:address 0x0000000000000000000000000000000000000403
interface LivingAssets {
    /// @notice Event emitted when a new collection is created
    /// @param collectionId the id of the newly created collection
    /// @param owner the owner of the newly created collection
    event NewCollection(uint64 collectionId, address indexed owner);

    /// @notice Creates a new collection
    /// @dev Call this function to create a new collection
    /// @param owner the owner of the newly created collection
    /// @return collectionId the id of the newly created collection
    function createCollection(address owner) external returns (uint64);
}
