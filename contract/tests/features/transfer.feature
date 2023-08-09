Feature: Transfer feature

  Scenario: The transfer cleans all the existing records
    Given a meta names contract
    And Alice minted 'meta.name' domain without a parent
    And Alice minted the 'Wallet' record with 'data' data for the 'meta.name' domain
    When Alice transfers the 'meta.name' domain to Bob
    Then Bob owns 'meta.name' domain
    And 'meta.name' domain does not have a 'Wallet' record
