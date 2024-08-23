// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @title Pallet Asset Metadata Extender Interface
/// @author LAOS Team
/// @notice This interface allows Solidity contracts to interact with pallet-asset-metadata-extender
/// @custom:address 0x0000000000000000000000000000000000000405
interface AssetMetadataExtender {
    /// @notice Emitted when a token metadata is extended
    /// @notice The emitted universal location and tokenURI have not undergone
    /// @notice any on-chain validation. Users are fully responsible for accuracy,
    /// @notice authenticity and preventing potential misuse or exploits.
    /// @param _claimer the address of the caller
    /// @param _universalLocationHash keccak256 hash of the universal location
    /// @param _universalLocation The universal location of the token
    /// @param _tokenURI the extended URI of the token
    event ExtendedULWithExternalURI(
        address indexed _claimer,
        bytes32 indexed _universalLocationHash,
        string _universalLocation,
        string _tokenURI
    );

    /// @notice Emitted when an extended token's URI is updated
    /// @notice The emitted universal location and tokenURI have not undergone
    /// @notice any on-chain validation. Users are fully responsible for accuracy,
    /// @notice authenticity and preventing potential misuse or exploits.
    /// @param _claimer the address of the user who updated the token URI
    /// @param _universalLocationHash keccak256 hash of the universal location
    /// @param _universalLocation the universal location of the token
    /// @param _tokenURI the URI of the extension after the update
    event UpdatedExtendedULWithExternalURI(
        address indexed _claimer,
        bytes32 indexed _universalLocationHash,
        string _universalLocation,
        string _tokenURI
    );

    /// @notice Extends the metadata of a token
    /// @notice The universal location and tokenURI provided to this method do not undergo
    /// @notice any on-chain validation. Users are fully responsible for accuracy,
    /// @notice authenticity and preventing potential misuse or exploits.
    /// @dev Emits the ExtendedULWithExternalURI event upon success
    /// @dev Reverts if the UL has been extended previously
    /// @param _uloc the Universal Location as a string identifying the token
    /// @param _tokenURI the URI of the extended metadata
    function extendULWithExternalURI(
        string calldata _uloc,
        string calldata _tokenURI
    ) external;

    /// @notice Updates the URI of an extended token
    /// @notice The universal location and tokenURI provided to this method do not undergo
    /// @notice any on-chain validation. Users are fully responsible for accuracy,
    /// @notice authenticity and preventing potential misuse or exploits.
    /// @param _uloc The universal location identifier of the token
    /// @param _tokenURI The new URI to be set for the token
    function updateExtendedULWithExternalURI(
        string calldata _uloc,
        string calldata _tokenURI
    ) external;

    /// @notice Returns the number of extensions made about a UL
    /// @param _uloc The Universal Location as a string identifying the asset
    /// @return The number of extensions
    function balanceOfUL(string calldata _uloc) external view returns (uint32);

    /// @notice Returns the claimer for an extension at a given index
    /// @param _uloc The Universal Location string identifying the asset
    /// @param _index The index of the extension
    /// @return The address of the claimer
    function claimerOfULByIndex(
        string calldata _uloc,
        uint32 _index
    ) external view returns (address);

    /// @notice Returns the tokenURI for an extension at a given index
    /// @notice The tokenURI returned by this method has not undergone
    /// @notice any on-chain validation. Users are fully responsible for accuracy,
    /// @notice authenticity and preventing potential misuse or exploits.
    /// @param _uloc The Universal Location string identifying the asset
    /// @param _index The index of the extension
    /// @return The tokenURI of the extension
    function extensionOfULByIndex(
        string calldata _uloc,
        uint32 _index
    ) external view returns (string memory);

    /// @notice Returns the extension of a Universal Location made by a claimer
    /// @notice The extension returned by this method has not undergone
    /// @notice any on-chain validation. Users are fully responsible for accuracy,
    /// @notice authenticity and preventing potential misuse or exploits.
    /// @dev Reverts if the Universal Location has no extension by the provided claimer
    /// @param _universalLocation The Universal Location
    /// @param _claimer The address of the claimer
    /// @return The tokenURI of the extension by the provided claimer
    function extensionOfULByClaimer(
        string calldata _universalLocation,
        address _claimer
    ) external view returns (string memory);

    /// @notice Checks if a Universal Location has an extension by a claimer
    /// @param _universalLocation The Universal Location
    /// @param _claimer The address of the claimer
    /// @return True if the Universal Location has an extension by the provided claimer, false otherwise
    function hasExtensionByClaimer(
        string calldata _universalLocation,
        address _claimer
    ) external view returns (bool);
}
