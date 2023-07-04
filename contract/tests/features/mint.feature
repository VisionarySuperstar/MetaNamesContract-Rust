Feature: Mint feature

  Scenario: The mint without the parent happens properly
    Given a meta names contract
    When Alice mints 'meta.name' domain without a parent
    Then Alice owns 'meta.name' domain

  Scenario: The mint with the owned parent happens properly
    Given a meta names contract
    When Alice mints 'meta.name' domain without a parent
    And Alice mints 'meta.name.sub' domain with 'meta.name' domain as the parent
    Then Alice owns 'meta.name.sub' domain

  Scenario: The mint with the approved parent happens properly
    Given a meta names contract
    When Alice mints 'meta.name' domain without a parent
    And Alice approves Bob on 'meta.name' domain
    And Bob mints 'meta.name.sub' domain with 'meta.name' domain as the parent
    Then Bob owns 'meta.name.sub' domain

  Scenario: The mint with not owned parent does not happen
    Given a meta names contract
    When  Alice mints 'meta.name' domain without a parent
    And Bob mints 'meta.name.sub' domain with 'meta.name' domain as the parent
    Then 'meta.name.sub' domain is not minted

  Scenario: The mint with the owned but not coherent parent does not happen
    Given a meta names contract
    When Alice mints 'name.meta' domain without a parent
    And Alice mints 'meta.random' domain with 'name.meta' domain as the parent
    Then 'meta.random' domain is not minted
