Feature: Mint feature

  # NOTE: Cannot make integration tests on normal mint as there are minting fees
  Scenario: The mint without fees without the parent occurs properly
    Given a meta names contract
    When Alice mints 'mpc.name' domain without fees and a parent
    Then Alice owns 'mpc.name' domain

  Scenario: The mint does not occur when the contract is disabled
    Given a meta names contract
    And Alice user with the admin role
    And contract config 'contract_enabled' is 'false'
    When Alice mints 'mpc.name' domain without a parent
    Then 'mpc.name' domain is not minted

  Scenario: The minting process of a domain without any parent, carried out by an administrator user, is executed correctly
    Given a meta names contract
    And Alice user with the admin role
    When Alice mints 'mpc.name' domain without a parent
    Then Alice owns 'mpc.name' domain
    And Alice mint count is 1

  Scenario: The minting process of a domain without any parent, carried out by an administrator user and when mint count limit 0, is executed correctly
    Given a meta names contract
    And Alice user with the admin role
    And contract config 'mint_count_limit_enabled' is 'true'
    And contract config 'mint_count_limit' is '0'
    When Alice mints 'mpc.name' domain without a parent
    Then Alice owns 'mpc.name' domain
    And Alice mint count is 1

  Scenario: The minting process of a domain without any parent, with a user without the whitelist role, fails
    Given a meta names contract
    And contract config 'whitelist_enabled' is 'true'
    When Alice mints 'mpc.name' domain without a parent
    Then 'mpc.name' domain is not minted

  Scenario: The minting process of a domain without any parent, with mint count limit 0, fails
    Given a meta names contract
    And contract config 'mint_count_limit_enabled' is 'true'
    And contract config 'mint_count_limit' is '0'
    When Alice mints 'mpc.name' domain without a parent
    Then 'mpc.name' domain is not minted

  Scenario: The minting process of a domain without any parent, with a wrong payment token id, fails
    Given a meta names contract
    When Alice mints 'mpc.name' domain with 1 as payment token id and without a parent
    Then 'mpc.name' domain is not minted

  Scenario: The mint with the owned parent occurs properly
    Given a meta names contract
    And Alice minted 'mpc.name' domain without a parent
    When Alice mints 'mpc.name.sub' domain with 'mpc.name' domain as the parent
    Then Alice owns 'mpc.name.sub' domain

  Scenario: The mint with the approved parent occurs properly
    Given a meta names contract
    And Alice minted 'mpc.name' domain without a parent
    And Alice approved Bob on 'mpc.name' domain
    When Bob mints 'mpc.name.sub' domain with 'mpc.name' domain as the parent
    Then Bob owns 'mpc.name.sub' domain

  Scenario: The mint with a not owned parent does not happen
    Given a meta names contract
    And Alice minted 'mpc.name' domain without a parent
    When Bob mints 'mpc.name.sub' domain with 'mpc.name' domain as the parent
    Then 'mpc.name.sub' domain is not minted

  Scenario: The mint with the owned but not coherent parent does not happen
    Given a meta names contract
    And Alice minted 'mpc.name' domain without a parent
    When Alice mints 'mpc.random' domain with 'mpc.name' domain as the parent
    Then 'mpc.random' domain is not minted

  Scenario: The mint of a 35 chars long domain does not happen
    Given a meta names contract
    When Alice mints 'this.is.a.too.long.domain.meta.name' domain without fees and a parent
    Then 'this.is.a.too.long.domain.meta.name' domain is not minted

  # NOTE: Cannot make integration tests on normal mint as there are minting fees, thus requires admin user
  Scenario: The batch mint without fees without the parent occurs properly
    Given a meta names contract
    And Alice user with the admin role
    When Alice batch mints 'meta.name' and 'meta.test' domain without fees and a parent
    Then Alice owns 'meta.name' domain
    And Alice owns 'meta.test' domain
