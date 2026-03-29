// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title RiseTreasury
 * @dev Manages protocol funds and community-led grants for the RISE Protocol.
 */
contract RiseTreasury {
    address public governor;
    uint256 public totalFunds;

    event Deposit(address indexed from, uint256 amount);
    event ProposalExecuted(address indexed to, uint256 amount, string reason);
    event GovernanceTransferred(address indexed previousGovernor, address indexed newGovernor);

    modifier onlyGovernor() {
        require(msg.sender == governor, "RISE: Only Governor can execute");
        _;
    }

    constructor() {
        governor = msg.sender; // The deployer starts as the Governor
    }

    // Accept funds from the $RYZE tax or donations
    receive() external payable {
        totalFunds += msg.value;
        emit Deposit(msg.sender, msg.value);
    }

    // Executing a community grant (e.g., Marketing, Development, Rewards)
    function executeGrant(address payable _to, uint256 _amount, string memory _reason) 
        external 
        onlyGovernor 
    {
        require(_amount <= address(this).balance, "RISE: Insufficient treasury funds");
        
        _to.transfer(_amount);
        totalFunds -= _amount;
        
        emit ProposalExecuted(_to, _amount, _reason);
    }

    // Change who the Governor is (e.g., passing control to a DAO contract)
    function transferGovernance(address _newGovernor) external onlyGovernor {
        require(_newGovernor != address(0), "RISE: New governor is the zero address");
        emit GovernanceTransferred(governor, _newGovernor);
        governor = _newGovernor;
    }

    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }
}
