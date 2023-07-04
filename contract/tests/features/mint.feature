Feature: Mint feature

  Scenario: The mint without the parent happens properly
    Given a meta names contract
    When Alice mints 'name.meta' domain without a parent
    Then Alice owns 'name.meta' domain

  Scenario: The mint with the owned parent happens properly
    Given a meta names contract
    When Alice mints 'name.meta' domain without a parent
    And Alice mints 'sub.name.meta' domain with 'name.meta' domain as the parent
    Then Alice owns 'sub.name.meta' domain

  Scenario: The mint with the approved parent happens properly
    Given a meta names contract
    When Alice mints 'name.meta' domain without a parent
    And Alice approves Bob on 'name.meta' domain
    And Bob mints 'sub.name.meta' domain with 'name.meta' domain as the parent
    Then Bob owns 'sub.name.meta' domain

  Scenario: The mint with not owned parent does not happen
    Given a meta names contract
    When  Alice mints 'name.meta' domain without a parent
    And Bob mints 'sub.name.meta' domain with 'name.meta' domain as the parent
    Then 'sub.name.meta' domain is not minted
