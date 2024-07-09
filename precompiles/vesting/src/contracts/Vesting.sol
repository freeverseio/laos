// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @title Pallet Vesting Interface (pallet code here: https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/vesting)
/// @author LAOS Team
/// @notice This interface allows Solidity contracts to interact with pallet-vesting
/// @custom:address 0x0000000000000000000000000000000000000406
interface Vesting {

    /// @dev Defines the vesting info
    /// @dev `locked` is the amount of locked tokens
    /// @dev `perBlock` is the amount of unlocking tokens per block
    /// @dev `startingBlock` is the block number at which the tokens start unlocking
    struct VestingInfo {
        uint256 locked;
        uint256 perBlock;
        uint256 startingBlock;
    }

    /// @notice Returns the vesting info of an account
    /// @param _target The address of the account the vesting data should be returned for
    /// @return The vesting info of the target account, as an array of `VestingInfo` structs
    function vesting(address _target) external view returns (VestingInfo[] memory);

    /// @notice Unlock the vested funds of the caller
    /// @dev Reverts if the caller doesn't have any vested funds
    function vest() external;

    /// @notice Unlock vested funds for the target account
    /// @dev Reverts if the target account doesn't have any vested funds
    /// @param _target The address for which funds will be unlocked
    function vestOther(address _target) external;
}
