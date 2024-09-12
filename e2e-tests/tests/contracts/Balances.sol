// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title Contract for keeping track of balances assigned to accounts
 * @author Freeverse.io, www.freeverse.io
 * @dev This contract uses OpenZeppelin's ERC20 implementation, but restricts
 *  all possibility to transfer any tokens, and adds the power to the contract
 *  owner to modify the balances assigned to accounts. The reasoning and logic
 *  for maintaining these balances are outside the scope of this code.
 */

contract Balances is ERC20, Ownable {

    mapping(address account => bool) public isAdmin;

    error TransfersNotSupported();
    error SenderIsNotAdmin(address sender);

    modifier onlyAdmin() {
        if (!isAdmin[msg.sender]) {
            revert SenderIsNotAdmin(msg.sender);
        }
        _;
    }

    modifier alwaysReverts() {
        _;
        revert TransfersNotSupported();
    }

    constructor(string memory name, string memory symbol) ERC20(name, symbol) Ownable(msg.sender) {}

    /**
     * @notice Allows the provided address to modify balances
     * @dev Only the contract owner is authorized
     * @param newAdmin the new address to be allowed
     */
    function allowAdmin(address newAdmin) external onlyOwner {
        isAdmin[newAdmin] = true;
    }

    /**
     * @notice Disallows the provided address to modify balances
     * @dev Only the contract owner is authorized
     * @param newAdmin the new address to be disallowed
     */
    function disallowAdmin(address newAdmin) external onlyOwner {
        isAdmin[newAdmin] = false;
    }

    /**
     * @notice Adds the provided amount to the balance of the provided recipient address
     * @dev Only admins are authorized
     * @param amount amount to be added
     * @param recipient the address to receive the amount
     */
    function increaseBalanceOf(uint256 amount, address recipient) external onlyAdmin {
        _mint(recipient, amount);
    }

    /**
     * @notice Subtracts the provided amount from the balance of the provided recipient address
     * @dev Only admins are authorized
     * @param amount amount to be subtracted
     * @param account the address to subtract the amount from
     */
    function decreaseBalanceOf(uint256 amount, address account) external onlyAdmin {
        _burn(account, amount);
    }

    /**
     * @notice All methods related to transfer are disabled and will revert
     */
    
    function transfer(address to, uint256 value) public pure override alwaysReverts returns (bool) {}

    function approve(address spender, uint256 value) public pure override alwaysReverts returns (bool) {}

    function transferFrom(address from, address to, uint256 value) public pure override alwaysReverts returns (bool) {}

}
