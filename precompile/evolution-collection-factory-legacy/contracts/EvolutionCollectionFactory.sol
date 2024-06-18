// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @title Pallet Laos Evolution Interface
/// @author LAOS Team
/// @notice This interface allows Solidity contracts to interact with pallet-laos-evolution
/// @custom:address 0x0000000000000000000000000000000000000403
interface EvolutionCollectionFactory {
    /// @notice Event emitted when a new collection is created
    /// @param _owner the owner of the newly created collection
    /// @param _collectionAddress the address of the newly created collection
    event NewCollection(address indexed _owner, address _collectionAddress);

    /// @notice Creates a new collection
    /// @dev Call this function to create a new collection
    /// @param _owner the owner of the newly created collection
    /// @return the address of the newly created collection
    function createCollection(address _owner) external returns (address);
}
