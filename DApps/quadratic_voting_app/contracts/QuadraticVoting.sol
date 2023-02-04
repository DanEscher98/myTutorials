// SPDX-License-Identifier: MIT
pragma solidity >=0.4.22 <0.9.0;

contract QuadraticVoting {
  struct Proposal {
    address payable owner;
    uint amount;
    bytes32 title;
    string imageHash; // IPFS cid
    string description;
    mapping(address => uint) positiveVotes; // user => weight
    mapping(address => uint) negativeVotes; //user => weight
    uint totalPositiveWeight;
    uint totalNegativeWeight;
  } 
  uint constant public voteCost = 10_000_000_000;
  mapping(uint => Proposal) public proposals; // proposalId => id
  uint public proposalCount = 0; // also next proposalId

  event ProposalCreated(uint proposalId);
  event Voted(uint proposalId, uint weight, bool value);

  /// how much a voter has bet into a proposal
  function currentWeight(uint proposalId, address addr, bool isPositive) public view returns(uint) {
    if (isPositive) {
      return proposals[proposalId].positiveVotes[addr];
    } else {
      return proposals[proposalId].negativeVotes[addr];
    }
  }

  /// calc the new cost of bet
  function calcCost(uint currWeight, uint weight) public pure returns(uint) {
    if (currWeight > weight) {
      return weight * weight * voteCost; // cost is always quadratic
    } else if (currWeight < weight) {
      // this allows users to save on costs if tey are increasing their vote
      return (weight * weight - currWeight * currWeight) * voteCost;
    } else {
      return 0;
    }
  }

  function createProposal(bytes32 title, string memory imageHash, string memory description) public {
    uint proposalId = proposalCount++;
    Proposal storage proposal = proposals[proposalId];
    proposal.owner = payable(msg.sender);
    proposal.title = title;
    proposal.imageHash = imageHash;
    proposal.description = description;
    emit ProposalCreated(proposalId);
  }

  /// emit a vote for or against
  function emitVote(uint proposalId, uint weight, bool vote) public payable {
    Proposal storage proposal = proposals[proposalId];
    require(msg.sender != proposal.owner); // owners cannot vote on their own proposals
    
    uint currWeight = proposal.positiveVotes[msg.sender];
    if (currWeight == weight) {
      return; // no need to process further if vote has not changed
    }

    uint cost = calcCost(currWeight, weight);
    require(msg.value >= cost); //msg.value must be enough to cover the cost

    if (vote) { // vote for
      proposal.positiveVotes[msg.sender] = weight;
      proposal.totalPositiveWeight += weight - currWeight;
      
      // weight cannot be both positive and negative simultaneously
      proposal.totalNegativeWeight -= proposal.negativeVotes[msg.sender];
      proposal.negativeVotes[msg.sender] = 0;

      proposal.amount += msg.value; // reward creator of proposal for their contribution
    } else {    // vote against
      proposal.totalPositiveWeight -= proposal.positiveVotes[msg.sender];
      proposal.positiveVotes[msg.sender] = 0;

      // distribute voting cost to every proposal except for this one
      uint reward = msg.value / (proposalCount - 1);
      for (uint i = 0; i < proposalCount; i++) {
        if (i != proposalId) proposals[i].amount += reward;
      }
    }

    emit Voted(proposalId, weight, vote);
  }
  
  /// Allows the owner of a proposal to transfer any reward to their wallet
  function claim(uint proposalId) public {
    Proposal storage proposal = proposals[proposalId];
    require(msg.sender == proposal.owner);
    proposal.owner.transfer(proposal.amount);
    proposal.amount = 0;
  }
}
