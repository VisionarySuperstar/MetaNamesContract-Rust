Feature: Mint feature

  Scenario: If all required arguments are passed, the mint happens properly
    Given a meta names contract
    When Alice mints a domain without a parent
    Then Alice owns the domain
