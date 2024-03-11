Feature: Domain Record feature

  Scenario: The record mint occurs correctly
    Given a PNS contract
    And Alice minted 'mpc.name' domain without a parent
    When Alice mints the 'Wallet' record with 'data' data for the 'mpc.name' domain
    Then 'mpc.name' domain has a 'Wallet' record with 'data' data

  Scenario: The record mint of a not existing domain does not happen
    Given a PNS contract
    When Alice mints the 'Wallet' record with 'new-data' data for the 'mpc.name' domain
    Then 'mpc.name' domain does not exist
    And 'mpc.name' domain does not have a 'Wallet' record

  Scenario: The record mint of a deactivated domain does not happen
    Given a PNS contract
    And Alice minted 'mpc.name' domain without a parent
    And 'mpc.name' domain is expired
    When Alice mints the 'Wallet' record with 'data' data for the 'mpc.name' domain
    Then 'mpc.name' domain does not have a 'Wallet' record

  Scenario: The record mint of a domain with a deactivated parent does not happen
    Given a PNS contract
    And Alice minted 'mpc.name' domain without a parent
    And Alice minted 'mpc.name.sub' domain with 'mpc.name' domain as the parent
    And 'mpc.name' domain is expired
    When Alice mints the 'Wallet' record with 'data' data for the 'mpc.name.sub' domain
    Then 'mpc.name.sub' domain does not have a 'Wallet' record

  Scenario: The record mint of a domain with the deactivated root parent does not happen
    Given a PNS contract
    And Alice minted 'mpc.name' domain without a parent
    And Alice minted 'mpc.name.sub' domain with 'mpc.name' domain as the parent
    And Alice minted 'mpc.name.sub.sea' domain with 'mpc.name.sub' domain as the parent
    And 'mpc.name' domain is expired
    When Alice mints the 'Wallet' record with 'data' data for the 'mpc.name.sub.sea' domain
    Then 'mpc.name.sub.sea' domain does not have a 'Wallet' record

  Scenario: The record mint of an already existing record does not happen
    Given a PNS contract
    And Alice minted 'mpc.name' domain without a parent
    And Alice minted the 'Wallet' record with 'data' data for the 'mpc.name' domain
    When Alice mints the 'Wallet' record with 'new-data' data for the 'mpc.name' domain
    Then 'mpc.name' domain has a 'Wallet' record with 'data' data

  Scenario: The record update occurs correctly
    Given a PNS contract
    And Alice minted 'mpc.name' domain without a parent
    And Alice minted the 'Wallet' record with 'data' data for the 'mpc.name' domain
    When Alice updates the 'Wallet' record with 'new-data' data for the 'mpc.name' domain
    Then 'mpc.name' domain has a 'Wallet' record with 'new-data' data

  Scenario: The record update of a not existing domain does not happen
    Given a PNS contract
    When Alice updates the 'Wallet' record with 'new-data' data for the 'mpc.name' domain
    Then 'mpc.name' domain does not exist
    And 'mpc.name' domain does not have a 'Wallet' record

  Scenario: The record update of a not existing record does not happen
    Given a PNS contract
    And Alice minted 'mpc.name' domain without a parent
    When Alice updates the 'Wallet' record with 'new-data' data for the 'mpc.name' domain
    Then 'mpc.name' domain does not have a 'Wallet' record

  Scenario: The record delete occurs correctly
    Given a PNS contract
    And Alice minted 'mpc.name' domain without a parent
    And Alice minted the 'Wallet' record with 'data' data for the 'mpc.name' domain
    When Alice deletes the 'Wallet' record for the 'mpc.name' domain
    Then 'mpc.name' domain does not have a 'Wallet' record

  Scenario: The deletion of all records occurs correctly
    Given a PNS contract
    And Alice minted 'mpc.name' domain without a parent
    And Alice minted the 'Wallet' record with 'data' data for the 'mpc.name' domain
    And Alice minted the 'Twitter' record with 'handle' data for the 'mpc.name' domain
    When Alice deletes all records for the 'mpc.name' domain
    Then 'mpc.name' domain does not have a 'Wallet' record
    And 'mpc.name' domain does not have a 'Twitter' record
