Feature: Domain feature

  Scenario: The domain mint without the parent occurs correctly
    Given a PNS contract
    When Alice mints 'mpc.name' domain without a parent
    Then 'mpc.name' domain is minted

  Scenario: The domain mint with an unexisting parent does not happen
    Given a PNS contract
    When Alice mints 'mpc.name' domain with 'not-existing' domain as the parent
    Then 'mpc.name' domain is not minted
