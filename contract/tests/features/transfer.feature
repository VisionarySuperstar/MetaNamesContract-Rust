Feature: Transfer feature

  Scenario: The transfer cleans all the existing records
    Given a meta names contract
    And Alice minted 'mpc.name' domain without a parent
    And Alice minted the 'Wallet' record with 'data' data for the 'mpc.name' domain
    When Alice transfers the 'mpc.name' domain to Bob
    Then Bob owns 'mpc.name' domain
    And 'mpc.name' domain does not have a 'Wallet' record
