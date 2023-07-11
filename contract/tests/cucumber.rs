use std::panic::catch_unwind;

use cucumber::{given, then, when, World};
use meta_names_contract::{
    contract::{approve_domain, initialize, mint},
    msg::InitMsg,
    state::ContractState,
};
use utils::tests::{mock_address, mock_contract_context};

const ALICE_ADDRESS: u8 = 1;
const BOB_ADDRESS: u8 = 2;

#[derive(Debug, Default, World)]
pub struct ContractWorld {
    state: ContractState,
}

fn get_address_for_user(user: String) -> u8 {
    match user.as_str() {
        "Alice" => ALICE_ADDRESS,
        "Bob" => BOB_ADDRESS,
        _ => panic!("Unknown user"),
    }
}

#[given("a meta names contract")]
fn meta_names_contract(world: &mut ContractWorld) {
    let msg = InitMsg {
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        uri_template: "metanames.io".to_string(),
    };

    let (state, _) = initialize(mock_contract_context(ALICE_ADDRESS), msg);
    world.state = state;
}

#[given(expr = "{word} minted '{word}' domain without a parent")]
#[when(expr = "{word} mints '{word}' domain without a parent")]
fn mint_a_domain(world: &mut ContractWorld, user: String, domain: String) {
    let res = catch_unwind(|| {
        mint(
            mock_contract_context(get_address_for_user(user.clone())),
            world.state.clone(),
            domain,
            mock_address(get_address_for_user(user)),
            None,
            None,
        )
    });

    if let Ok((new_state, _)) = res {
        world.state = new_state;
    }
}

#[given(expr = "{word} approved {word} on '{word}' domain")]
fn user_approve_domain(world: &mut ContractWorld, user: String, approved: String, domain: String) {
    let (new_state, _) = approve_domain(
        mock_contract_context(get_address_for_user(user)),
        world.state.clone(),
        Some(mock_address(get_address_for_user(approved))),
        domain,
    );

    world.state = new_state;
}

#[when(expr = "{word} mints '{word}' domain with '{word}' domain as the parent")]
fn mint_domain_with_parent(
    world: &mut ContractWorld,
    user: String,
    domain: String,
    parent: String,
) {
    let res = catch_unwind(|| {
        mint(
            mock_contract_context(get_address_for_user(user.clone())),
            world.state.clone(),
            domain,
            mock_address(get_address_for_user(user)),
            None,
            Some(parent),
        )
    });

    if let Ok((new_state, _)) = res {
        world.state = new_state;
    }
}

#[then(expr = "{word} owns '{word}' domain")]
fn owns_the_domain(world: &mut ContractWorld, user: String, domain: String) {
    let domain = world
        .state
        .pns
        .get_domain(&domain)
        .unwrap();

    assert_eq!(
        world.state.nft.owner_of(domain.token_id),
        mock_address(get_address_for_user(user))
    );
}

#[then(expr = "'{word}' domain is not minted")]
fn domain_is_not_minted(world: &mut ContractWorld, domain: String) {
    let domain = world
        .state
        .pns
        .get_domain(&domain);

    assert_eq!(domain, None);
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(ContractWorld::run("tests/features"));
}
