Feature: Renew domain

  Scenario: The renewal of a domain occurs properly
    Given a meta names contract
    And Alice user with the admin role
    And Alice minted 'meta.name' domain without a parent
    When Alice renews 'meta.name' domain for 2 years
    Then 'meta.name' domain expires in 2 years

  Scenario: The renewal of a domain with the wrong payment token id, fails
    Given a meta names contract
    And Alice minted 'meta.name' domain without a parent
    When Alice renews 'meta.name' domain with 1 payment token id for 2 years
    Then 'meta.name' domain does not expire in 2 years
