// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @title Pallet Laos Evolution Interface
/// @author LAOS Team
/// @notice This interface allows Solidity contracts to interact with pallet-laos-evolution
interface EvolutionCollection {
    /// @notice Emitted when a new token is minted
    /// @dev Id of the token is concatenation of `slot` and `to`
    /// @param _to the initial owner of the newly minted token
    /// @param _slot the slot of the token
    /// @param _tokenId the resulting id of the newly minted token
    /// @param _tokenURI the URI of the newly minted token
    event MintedWithExternalURI(
        address indexed _to,
        uint96 _slot,
        uint256 _tokenId,
        string _tokenURI
    );

    /// @notice Emitted when a token metadata is updated
    /// @param _tokenId the id of the token for which the metadata has changed
    /// @param _tokenURI the new URI of the token
    event EvolvedWithExternalURI(uint256 indexed _tokenId, string _tokenURI);

    /// @notice Emitted when ownership of the collection changes
    /// @param previousOwner the previous owner of the collection
    /// @param newOwner the new owner of the collection
    event OwnershipTransferred(
        address indexed previousOwner,
        address indexed newOwner
    );

    /// @notice Emitted when public minting is enabled for the collection
    event PublicMintingEnabled();

    /// @notice Emitted when public minting is disabled for the collection
    event PublicMintingDisabled();

    /// @notice Owner of the collection
    /// @dev Call this function to get the owner of a collection
    /// @return the owner of the collection
    function owner() external view returns (address);

    /// @notice Provides a distinct Uniform Resource Identifier (URI) for a given token within a specified collection.
    /// @dev Implementations must follow the ERC-721 standard for token URIs, which should point to a JSON file conforming to the "ERC721 Metadata JSON Schema".
    /// @param _tokenId The unique identifier of the token within the specified collection.
    /// @return A string representing the URI of the specified token.
    function tokenURI(uint256 _tokenId) external view returns (string memory);

    /// @notice Mint a new token
    /// @dev Call this function to mint a new token, the caller must be the owner of the collection
    /// @param _to the owner of the newly minted token
    /// @param _slot the slot of the token
    /// @param _tokenURI the tokenURI of the newly minted token
    /// @return the id of the newly minted token
    function mintWithExternalURI(
        address _to,
        uint96 _slot,
        string calldata _tokenURI
    ) external returns (uint256);

    /// @notice Changes the tokenURI of an existing token
    /// @dev Call this function to evolve an existing token, the caller must be the owner of the collection
    /// @param _tokenId the id of the token
    /// @param _tokenURI the new tokenURI of the token
    function evolveWithExternalURI(
        uint256 _tokenId,
        string calldata _tokenURI
    ) external;

    /// @notice Transfers ownership of the collection to a new account (`newOwner`).
    /// @dev Call this function to transfer ownership of the collection, the caller must be the owner of the collection
    /// @param newOwner The address to transfer ownership to.
    function transferOwnership(address newOwner) external;

    /// @notice Enables public minting for the collection
    /// When enabled, any address is allowed to mint on this collection
    /// This does not affect evolution: only the owner of the collection can continue evolving assets
    /// @dev Call this function to enable public minting for the collection, the caller must be the owner of the collection
    function enablePublicMinting() external;

    /// @notice Disables public minting for the collection
    /// @dev Call this function to disable public minting for the collection, the caller must be the owner of the collection
    function disablePublicMinting() external;

    /// @notice Checks if public minting is enabled for the collection
    /// @dev Call this function to check if public minting is enabled for the collection
    /// @return true if public minting is enabled for the collection, false otherwise
    function isPublicMintingEnabled() external view returns (bool);
}
