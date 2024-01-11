// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @title Pallet Asset Metadata Extender Interface
/// @author LAOS Team
/// @notice This interface allows Solidity contracts to interact with pallet-asset-metadata-extender
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
    /// @param _uloc The universal location identifier of the token as a string
    /// @param _tokenURI The new token URI after the update
    event ExtendedTokenURIUpdated(
        address indexed _claimer,
        string indexed _uloc,
        string _tokenURI
    );

    /// @notice Updates the URI of an extended token
    /// @param _uloc The universal location identifier of the token
    /// @param _tokenURI The new URI to be set for the token
    function updateExtendedTokenURI(
        string calldata _uloc,
        string calldata _tokenURI
    ) external;
}