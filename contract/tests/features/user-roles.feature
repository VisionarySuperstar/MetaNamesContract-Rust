Feature: User role feature

  Scenario: The granted admin user can grant other admin users
    Given a meta names contract with 'whitelist' enabled
    And Alice user with the admin role
    When Alice user grants the admin role for Bob user
    Then Bob user has the admin role

  Scenario: The granted admin user can grant whitelist to users
    Given a meta names contract with 'whitelist' enabled
    And Alice user with the admin role
    When Alice user grants the whitelist role for Bob user
    Then Bob user has the whitelist role

  Scenario: The denied admin user cannot grant other admin users
    Given a meta names contract with 'whitelist' enabled
    When Alice user grants the admin role for Bob user
    Then Bob user has not the admin role

  Scenario: The denied admin user cannot grant whitelist to users
    Given a meta names contract with 'whitelist' enabled
    When Alice user grants the whitelist role for Bob user
    Then Bob user has not the whitelist role
