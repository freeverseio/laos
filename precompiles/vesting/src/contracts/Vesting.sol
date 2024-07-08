// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @title Pallet Vesting Interface (pallet code here: https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/vesting)
/// @author LAOS Team
/// @notice This interface allows Solidity contracts to interact with pallet-vesting
interface Vesting {

    /// @dev Defines the vesting info
    struct VestingInfo {
        /// The amount of locked tokens
        uint256 locked;
        /// The amount of unlocking tokens per block
        uint256 perBlock;
        /// The block number at which the tokens start unlocking
        uint256 startingBlock;
    }

    /// @notice Returns the vesting info of an account
    /// @param _target The address of the account the vesting data should be returned for
    function vesting(address _target) external view returns (VestingInfo[] memory);

    /// @notice Unlock the vested funds of the caller
    /// @dev Reverts if the caller doesn't have any vested funds
    function vest() external;

    /// @notice Unlock vested funds for another account
    /// @dev Reverts if the target account doesn't have any vested funds
    /// @param _target The address the call should be made on behalf of
    function vestOther(address _target) external;
}
