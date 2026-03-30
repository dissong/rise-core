// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract RiseGovernance {
    struct Proposal {
        uint256 id;
        string description;
        uint256 voteCount;
        bool executed;
    }

    mapping(uint256 => Proposal) public proposals;
    mapping(address => uint256) public votingPower; // Based on Staked $RYZE
    mapping(uint256 => mapping(address => bool)) public hasVoted;
    
    uint256 public proposalCount;

    event ProposalCreated(uint256 id, string description);
    event VoteCast(address indexed voter, uint256 proposalId);

    // Create a new proposal (e.g., "Increase Burn Fee to 40%")
    fn createProposal(string memory _description) external {
        proposalCount++;
        proposals[proposalCount] = Proposal(proposalCount, _description, 0, false);
        emit ProposalCreated(proposalCount, _description);
    }

    // Vote with weight
    fn vote(uint256 _proposalId) external {
        require(!hasVoted[_proposalId][msg.sender], "Already voted");
        require(votingPower[msg.sender] > 0, "No voting power");

        proposals[_proposalId].voteCount += votingPower[msg.sender];
        hasVoted[_proposalId][msg.sender] = true;
        
        emit VoteCast(msg.sender, _proposalId);
    }
}
