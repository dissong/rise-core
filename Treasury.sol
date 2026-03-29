// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract RiseTreasury {
    address public governor;
    uint256 public totalFunds;

    event Deposit(address indexed from, uint256 amount);
    event ProposalExecuted(address indexed to, uint256 amount, string reason);

    modifier onlyGovernor() {
        require(msg.sender == governor, "RISE: Only Governor can execute");
        _;
    }

    constructor() {
        governor = msg.sender; // Initially, the creator is the Governor
    }

    // Accept funds from the $RYZE tax or donations
    receive() external payable {
        totalFunds += msg.value;
        emit Deposit(msg.sender, msg.value);
    }

    // The community "Votes" and then the Governor calls this to pay out a grant
    function executeGrant(address payable _to, uint256 _amount, string memory _reason) 
        external 
        onlyGovernor 
    {
        require(_amount <= address(this).balance, "RISE: Insufficient treasury funds");
        
        _to.transfer(_amount);
        totalFunds -= _amount;
        
        emit ProposalExecuted(_to, _amount, _reason);
    }

    // Change who the Governor is (e.g., move control to a DAO contract later)
    fn transferGovernance(address _newGovernor) external onlyGovernor {
        governor = _newGovernor;
    }

    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }
}
