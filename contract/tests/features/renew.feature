Feature: Renew domain

  Scenario: The renewal of a domain occurs properly
    Given a meta names contract
    And Alice user with the admin role
    And Alice minted 'meta.name' domain without a parent
    When Alice renews 'meta.name' domain for 2 years
    Then 'meta.name' domain expires in 2 years

