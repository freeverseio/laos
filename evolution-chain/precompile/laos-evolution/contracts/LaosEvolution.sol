// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @title Pallet Collection Manager Interface
/// @author LAOS Team
/// @notice This interface allows Solidity contracts to interact with pallet-living-assets
/// @custom:address 0x0000000000000000000000000000000000000403
interface LaosEvolution {
    /// @notice Event emitted when a new collection is created
    /// @param collectionId the id of the newly created collection
    /// @param owner the owner of the newly created collection
    event NewCollection(uint64 collectionId, address indexed owner);

    /// @notice Creates a new collection
    /// @dev Call this function to create a new collection
    /// @param owner the owner of the newly created collection
    /// @return the id of the newly created collection
    function createCollection(address owner) external returns (uint64);

    /// @notice Owner of the collection
    /// @dev Call this function to get the owner of a collection
    /// @param collectionId the id of the collection
    /// @return the owner of the collection
    function ownerOfCollection(
        uint64 collectionId
    ) external view returns (address);

    /// @notice Mint a new asset
    /// @dev Call this function to mint a new asset
    /// @param collectionId the id of the collection
    /// @param slot the slot of the asset
    /// @param to the owner of the newly minted asset
    /// @param tokenURI the tokenURI of the newly minted asset
    /// @return the id of the newly minted asset
    function mint(
        uint64 collectionId,
        uint96 slot,
        address to,
        string calldata tokenURI
    ) external returns (uint64);
}
