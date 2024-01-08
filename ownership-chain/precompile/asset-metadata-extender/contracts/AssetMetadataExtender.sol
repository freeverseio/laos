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
}