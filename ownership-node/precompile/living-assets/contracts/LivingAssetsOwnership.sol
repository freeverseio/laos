// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @author Freeverse team
/// @title Pallet LivingAssetsOwnership Interface
/// @dev The interface through which solidity contracts will interact with pallet-living-assets
/// @custom:address 0x0000000000000000000000000000000000000402
interface LivingAssets {
    /// @dev Create collection
    /// @custom:selector 0x1eaf2516
    ///
    /// @param collection_id The `collection_id` to be associated
    function createCollection(
        uint64 collection_id,
        address who
    ) external payable;

    /// @dev Get collection owner
    /// @custom:selector 0xfb34ae53
    ///
    /// @param collection_id The `collection_id`
    function ownerOfCollection(
        uint64 collection_id
    ) external view returns (bytes32);
}
