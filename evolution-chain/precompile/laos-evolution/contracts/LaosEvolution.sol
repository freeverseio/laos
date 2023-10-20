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
    function ownerOfCollection(uint64 collectionId) external view returns (address);

    /// @notice Provides a distinct Uniform Resource Identifier (URI) for a given token within a specified collection.
    /// @dev Implementations must follow the ERC-721 standard for token URIs, which should point to a JSON file conforming to the "ERC721 Metadata JSON Schema".
    /// @param collectionId The unique identifier of the collection to which the token belongs.
    /// @param tokenId The unique identifier of the token within the specified collection.
    /// @return A string representing the URI of the specified token.
    function tokenURI(uint64 collectionId, uint256 tokenId) external view returns (string memory);
}
