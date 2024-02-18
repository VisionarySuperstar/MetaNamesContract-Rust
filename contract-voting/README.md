# Voting
Smart contract implementing a simple majority open ballot vote for a proposal among a fixed list of eligible voters.
The public vote has a proposal id, a list of accounts that can participate, as well as a list of votes. The state of the contract shows the result, participants and if the vote is finished.

How it works
* The owner of the proposal deploys a Vote smart contract to the blockchain and initializes it.
* Eligible voters can cast their vote until the deadline.
* New voters can be added along the way.
* After the deadline passes anyone can initiate counting of the votes.
