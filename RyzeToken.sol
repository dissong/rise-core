// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title RYZE Token - Core Asset of RISE Protocol
 * @dev Implements fixed supply and automated deflationary burn.
 */
contract RyzeToken is ERC20, ERC20Burnable, Ownable {
    uint256 public constant MAX_SUPPLY = 100_000_000 * 10**18;
    uint256 public burnRate = 10; // 0.1% burn on transfers (10/10000)
    
    constructor() ERC20("Ryze Token", "RYZE") Ownable(msg.sender) {
        _mint(msg.sender, MAX_SUPPLY);
    }

    /**
     * @dev Overrides transfer to include the protocol burn mechanism.
     * This makes RYZE deflationary as the network scales.
     */
    function _update(address from, address to, uint256 value) internal virtual override {
        if (from != address(0) && to != address(0) && from != owner()) {
            uint256 burnAmount = (value * burnRate) / 10000;
            super._update(from, address(0), burnAmount); // Burn tokens
            super._update(from, to, value - burnAmount); // Send remaining
        } else {
            super._update(from, to, value);
        }
    }

    // Allow the DAO/Owner to adjust burn rate based on network congestion
    function setBurnRate(uint256 newRate) external onlyOwner {
        require(newRate <= 200, "Burn rate too high"); // Max 2%
        burnRate = newRate;
    }
}
