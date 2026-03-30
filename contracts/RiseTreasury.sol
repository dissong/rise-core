// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract RiseTreasury {
    // State Variables
    address public daoTreasury;
    uint256 public totalBurned;
    
    // Distribution Percentages (Total 100)
    uint256 public constant BURN_FEE = 30;
    uint256 public constant DAO_FEE = 20;
    uint256 public constant VALIDATOR_FEE = 50;

    event FeesDistributed(uint256 burned, uint256 daoAmount, uint256 validatorAmount);

    constructor(address _dao) {
        daoTreasury = _dao;
    }

    // This function handles the incoming transaction fees
    function distributeFees(address _validator) external payable {
        uint256 amount = msg.value;
        require(amount > 0, "No fees to distribute");

        // Calculate Splits
        uint256 bAmount = (amount * BURN_FEE) / 100;
        uint256 dAmount = (amount * DAO_FEE) / 100;
        uint256 vAmount = (amount * VALIDATOR_FEE) / 100;

        // 1. "Burn" the tokens (In this simple version, we send to 0x0 address)
        totalBurned += bAmount;
        
        // 2. Send to DAO Treasury
        (bool successDao, ) = payable(daoTreasury).call{value: dAmount}("");
        require(successDao, "DAO transfer failed");

        // 3. Send to Validator
        (bool successVal, ) = payable(_validator).call{value: vAmount}("");
        require(successVal, "Validator transfer failed");

        emit FeesDistributed(bAmount, dAmount, vAmount);
    }
}
