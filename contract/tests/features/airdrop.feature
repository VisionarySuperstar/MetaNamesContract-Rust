Feature: Airdrop feature

  Scenario: An user with airdrop role can add airdrop
    Given a meta names contract
    And Alice user with the airdrop role
    When Alice add airdrop to 'Bob'
    Then Bob has the airdrop

  Scenario: An user without airdrop role cannot add airdrop
    Given a meta names contract
    When Alice add airdrop to 'Bob'
    Then Bob has not the airdrop
