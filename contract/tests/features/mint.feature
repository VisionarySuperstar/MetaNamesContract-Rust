Feature: Mint feature

  // NOTE: Cannot make integration tests on normal mint as there are minting fees
  Scenario: The mint without fees without the parent occurs properly
    Given a meta names contract
    When Alice mints 'meta.name' domain without fees and a parent
    Then Alice owns 'meta.name' domain

  Scenario: The minting process of a domain without any parent, carried out by an administrator user, is executed correctly
    Given a meta names contract
    And Alice user with the admin role
    When Alice mints 'meta.name' domain without a parent
    Then Alice owns 'meta.name' domain
    And Alice mint count is 1

  Scenario: The minting process of a domain without any parent, carried out by an administrator user and when mint count limit 0, is executed correctly
    Given a meta names contract
    And Alice user with the admin role
    And contract config 'mint_count_limit_enabled' is 'true'
    And contract config 'mint_count_limit' is '0'
    When Alice mints 'meta.name' domain without a parent
    Then Alice owns 'meta.name' domain
    And Alice mint count is 1

  Scenario: The minting process of a domain without any parent, with a user without the whitelist role, fails
    Given a meta names contract
    When Alice mints 'meta.name' domain without a parent
    Then 'meta.name' domain is not minted

  Scenario: The minting process of a domain without any parent, with a user with the whitelist role, but with mint count limit 0, fails
    Given a meta names contract
    And contract config 'whitelist_enabled' is 'false'
    And contract config 'mint_count_limit_enabled' is 'true'
    And contract config 'mint_count_limit' is '0'
    When Alice mints 'meta.name' domain without a parent
    Then 'meta.name' domain is not minted

  Scenario: The mint with the owned parent occurs properly
    Given a meta names contract
    And Alice minted 'meta.name' domain without a parent
    When Alice mints 'meta.name.sub' domain with 'meta.name' domain as the parent
    Then Alice owns 'meta.name.sub' domain

  Scenario: The mint with the approved parent occurs properly
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
