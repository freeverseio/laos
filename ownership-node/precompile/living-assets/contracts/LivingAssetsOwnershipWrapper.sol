// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;
import "./LivingAssetsOwnership.sol";

contract A {
    LivingAssets public constant LIVING_ASSETS =
        LivingAssets(0x0000000000000000000000000000000000000402);

    /// @notice Get owner of collection
    function ownerOfCollection(
        uint64 collection_id
    ) public view returns (bytes32) {
        return LIVING_ASSETS.ownerOfCollection(collection_id);
    }

    /// @notice Create collection
    function createCollection(
        uint64 collection_id,
        address who
    ) public payable {
        LIVING_ASSETS.createCollection(collection_id, who);
    }
}
