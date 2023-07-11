Feature: Domain feature

  Scenario: The domain mint without the parent happens correctly
    Given a PNS contract
    When Alice mints 'meta.name' domain without a parent
    Then 'meta.name' domain is minted

  Scenario: The domain mint with an unexisting parent does not happen
    Given a PNS contract
    When Alice mints 'meta.name' domain with 'not-existing' domain as the parent
    Then 'meta.name' domain is not minted
