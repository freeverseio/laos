// SPDX-License-Identifier: MIT
// derived from OpenZeppelin Contracts (last updated v4.9.0) (token/ERC721/IERC721.sol)
pragma solidity >=0.8.3;

interface IERC721 {
    /**
     * @dev See {IERC721Metadata-tokenURI}.
     */
    function tokenURI(uint256 _tokenId) external view returns (string memory);

    function ownerOf(uint256 _tokenId) external view returns (address);

    event Transfer(address indexed _from, address indexed _to, uint256 indexed _tokenId);

    function transferFrom(address _from, address _to, uint256 _tokenId) external;
}
