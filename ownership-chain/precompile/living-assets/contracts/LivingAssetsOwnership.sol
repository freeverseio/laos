// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @title Pallet Collection Manager Interface
/// @author Freeverse team
/// @notice This interface allows Solidity contracts to interact with pallet-living-assets
/// @custom:address 0x0000000000000000000000000000000000000402
interface LivingAssets {
    /// @notice Event emitted when a new collection is created
    /// @param collectionAddress Address of the newly created ERC721 collection
    event CreateCollection(address indexed collectionAddress);

    /// @notice Creates a new collection
    /// @dev Call this function to create a new collection
    /// @return address of the ERC721 collection
    function createCollection(string memory baseURI) external returns (address);
}
