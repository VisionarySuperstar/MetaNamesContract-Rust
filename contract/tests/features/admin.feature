Feature: Admin feature

  Scenario: The granted admin user can grant other admin users
    Given a meta names contract
    And Alice user with the admin role
    When Alice user grants the admin role for Bob user
    Then Bob user is an admin

  Scenario: The denied admin user cannot grant other admin users
    Given a meta names contract
    When Alice user grants the admin role for Bob user
    Then Bob user is not an admin


