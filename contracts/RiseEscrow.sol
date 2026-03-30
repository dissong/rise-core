// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract RiseEscrow {
    struct Deposit {
        uint256 amount;
        uint256 releaseTime;
        bool claimed;
    }

    mapping(address => Deposit) public deposits;
    uint256 public constant UNBONDING_PERIOD = 7 days;

    event FundsLocked(address indexed user, uint256 amount, uint256 releaseTime);
    event FundsReleased(address indexed user, uint256 amount);

    // Lock funds for the unbonding period
    function lockFunds() external payable {
        require(msg.value > 0, "Cannot lock 0");
        deposits[msg.sender] = Deposit({
            amount: msg.value,
            releaseTime: block.timestamp + UNBONDING_PERIOD,
            claimed: false
        });
        emit FundsLocked(msg.sender, msg.value, deposits[msg.sender].releaseTime);
    }

    // Release only after time has passed
    function withdraw() external {
        Deposit storage d = deposits[msg.sender];
        require(block.timestamp >= d.releaseTime, "Funds still locked");
        require(!d.claimed, "Already withdrawn");
        require(d.amount > 0, "No funds found");

        d.claimed = true;
        payable(msg.sender).transfer(d.amount);
        emit FundsReleased(msg.sender, d.amount);
    }
}
