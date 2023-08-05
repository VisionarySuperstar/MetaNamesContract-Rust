Feature: Mint feature

  // NOTE: Cannot make integration tests on normal mint as there are minting fees
  Scenario: The mint without fees without the parent happens properly
    Given a meta names contract
    When Alice mints 'meta.name' domain without fees and a parent
    Then Alice owns 'meta.name' domain

  Scenario: The minting process of a domain without any parent, carried out by an administrator user, is executed correctly
    Given a meta names contract
    And Alice user with the admin role
    When Alice mints 'meta.name' domain without a parent
    Then Alice owns 'meta.name' domain

  Scenario: The mint with the owned parent happens properly
    Given a meta names contract
    And Alice minted 'meta.name' domain without a parent
    When Alice mints 'meta.name.sub' domain with 'meta.name' domain as the parent
    Then Alice owns 'meta.name.sub' domain

  Scenario: The mint with the approved parent happens properly
    Given a meta names contract
    And Alice minted 'meta.name' domain without a parent
    And Alice approved Bob on 'meta.name' domain
    When Bob mints 'meta.name.sub' domain with 'meta.name' domain as the parent
    Then Bob owns 'meta.name.sub' domain

  Scenario: The mint with a not owned parent does not happen
    Given a meta names contract
    And Alice minted 'meta.name' domain without a parent
    When Bob mints 'meta.name.sub' domain with 'meta.name' domain as the parent
    Then 'meta.name.sub' domain is not minted

  Scenario: The mint with the owned but not coherent parent does not happen
    Given a meta names contract
    And Alice minted 'meta.name' domain without a parent
    When Alice mints 'meta.random' domain with 'meta.name' domain as the parent
    Then 'meta.random' domain is not minted

  Scenario: The mint of a 35 chars long domain does not happen
    Given a meta names contract
    When Alice mints 'this.is.a.too.long.domain.meta.name' domain without fees and a parent
    Then 'this.is.a.too.long.domain.meta.name' domain is not minted
