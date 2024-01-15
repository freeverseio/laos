// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @title Pallet Asset Metadata Extender Interface
/// @author LAOS Team
/// @notice This interface allows Solidity contracts to interact with pallet-asset-metadata-extender
/// @custom:address 0x0000000000000000000000000000000000000404
interface AssetMetadataExtender {
    /// @notice Emitted when a token metadata is extended
    /// @param _claimer the address of the caller
    /// @param _universalLocation the universal location of the token
    /// @param _tokenURI the extended URI of the token
    event TokenURIExtended(
        address indexed _claimer,
        uint256 indexed _universalLocation,
        string _tokenURI
    );

    /// @notice Emitted when an extended token's URI is updated
    /// @param _claimer The address of the user who updated the token URI
    /// @param _universelLocationHash keccak256 hash of the universal location
    /// @param _universalLocation The universal location of the token
    /// @param _tokenURI The new token URI after the update
    event ExtendedTokenURIUpdated(
        address indexed _claimer,
        uint256 indexed _universelLocationHash,
        string _universalLocation,
        string _tokenURI
    );

    /// @notice Updates the URI of an extended token
    /// @param _uloc The universal location identifier of the token
    /// @param _tokenURI The new URI to be set for the token
    function updateExtendedTokenURI(
        string calldata _uloc,
        string calldata _tokenURI
    ) external;

    /// @notice Returns the number of extensions made about a UL
    /// @param uloc The Universal Location as a string identifying the asset
    /// @return The number of extensions
    function balanceOfUL(string calldata uloc) external view returns (uint256);

    /// @notice Returns the claimer for an extension at a given index
    /// @param uloc The Universal Location string identifying the asset
    /// @param index The index of the extension
    /// @return The address of the claimer
    function claimerOfULByIndex(
        string calldata uloc,
        uint256 index
    ) external view returns (address);

    /// @notice Returns the tokenURI for an extension at a given index
    /// @param uloc The Universal Location string identifying the asset
    /// @param index The index of the extension
    /// @return The tokenURI of the extension
    function extensionOfULByIndex(
        string calldata uloc,
        uint256 index
    ) external view returns (string memory);
}
