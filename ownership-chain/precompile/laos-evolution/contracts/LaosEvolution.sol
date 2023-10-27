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

    /// @notice Emitted when a new token is minted
    /// @dev Id of the token is concatenation of `slot` and `to`
    /// @param collectionId the id of the collection
    /// @param slot the slot of the token
    /// @param to the initial owner of the newly minted token
    /// @param tokenURI the URI of the newly minted token
    /// @param tokenId the resulting id of the newly minted token
    event MintedWithExternalTokenURI(
        uint64 collectionId,
        uint96 slot,
        address indexed to,
        string tokenURI,
        uint256 tokenId
    );

    /// @notice Emitted when a token metadata is updated
    /// @param tokenId the id for which the metadata has changed
    /// @param collectionId the id of the collection
    /// @param tokenURI the new URI of the token
    event MetadataUpdate(
        uint256 tokenId,
        uint64 collectionId,
        string tokenURI
    );

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

    /// @notice Mint a new token
    /// @dev Call this function to mint a new token, the caller must be the owner of the collection
    /// @param collectionId the id of the collection
    /// @param slot the slot of the token
    /// @param to the owner of the newly minted token
    /// @param tokenURI the tokenURI of the newly minted token
    /// @return the id of the newly minted token
    function mintWithExternalUri(
        uint64 collectionId,
        uint96 slot,
        address to,
        string calldata tokenURI
    ) external returns (uint256);

    /// @notice Changes the tokenURI of an existing token
    /// @dev Call this function to evolve an existing token, the caller must be the owner of the collection
    /// @param collectionId the id of the collection
    /// @param tokenId the id of the token
    /// @param tokenURI the new tokenURI of the token
    function evolveWithExternalUri(
        uint64 collectionId,
        uint256 tokenId,
        string calldata tokenURI
    ) external returns (uint256);
}
