use cucumber::{given, then, when, World};
use meta_names_contract::{
    contract::{approve, initialize, mint},
    msg::InitMsg,
    state::ContractState,
};
use utils::tests::{mock_address, mock_contract_context, string_to_bytes};

const DEFAULT_DOMAIN_NAME: &str = "name.meta";
const DEFAULT_SUBDOMAIN_NAME: &str = "sub.name.meta";
const ALICE_ADDRESS: u8 = 1;
const BOB_ADDRESS: u8 = 2;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct ContractWorld {
    state: ContractState,
}

#[given("a meta names contract")]
fn meta_names_contract(world: &mut ContractWorld) {
    let msg = InitMsg {
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        uri_template: "metanames.io".to_string(),
    };

    let (state, _) = initialize(mock_contract_context(2), msg);
    world.state = state;
}

#[when("Alice mints 'name.meta' domain without a parent")]
fn alice_mint_a_domain(world: &mut ContractWorld) {
    let (new_state, _) = mint(
        mock_contract_context(2),
        world.state.clone(),
        string_to_bytes(DEFAULT_DOMAIN_NAME),
        mock_address(ALICE_ADDRESS),
        None,
        None,
    );

    world.state = new_state;
}

#[when("Alice mints 'sub.name.meta' domain with 'name.meta' domain as the parent")]
fn alice_mint_subdomain_with_parent(world: &mut ContractWorld) {
    let (new_state, _) = mint(
        mock_contract_context(2),
        world.state.clone(),
        string_to_bytes(DEFAULT_SUBDOMAIN_NAME),
        mock_address(ALICE_ADDRESS),
        None,
        Some(string_to_bytes(DEFAULT_DOMAIN_NAME)),
    );

    world.state = new_state;
}

#[when("Bob mints 'sub.name.meta' domain with Alice's 'name.meta' domain as the parent")]
fn bob_mints_subdomain_with_parent(world: &mut ContractWorld) {
    let (new_state, _) = mint(
        mock_contract_context(2),
        world.state.clone(),
        string_to_bytes(DEFAULT_SUBDOMAIN_NAME),
        mock_address(BOB_ADDRESS),
        None,
        Some(string_to_bytes(DEFAULT_DOMAIN_NAME)),
    );

    world.state = new_state;
}

#[when("Alice mints a domain with a parent without owning it")]
fn mint_a_domain_with_parent_without_owning_it(world: &mut ContractWorld) {
    let (new_state, _) = mint(
        mock_contract_context(2),
        world.state.clone(),
        string_to_bytes(DEFAULT_DOMAIN_NAME),
        mock_address(ALICE_ADDRESS),
        None,
        None,
    );

    world.state = new_state;
}

#[when("Alice approves Bob on 'name.meta' domain")]
fn alice_approves_bob_on_domain(world: &mut ContractWorld) {
    // TODO: Add approve_domain function
    let token_id = world
        .state
        .pns
        .get_domain(string_to_bytes(DEFAULT_DOMAIN_NAME).as_slice())
        .unwrap()
        .token_id;

    let (new_state, _) = approve(
        mock_contract_context(2),
        world.state.clone(),
        Some(mock_address(BOB_ADDRESS)),
        token_id
    );

    world.state = new_state;
}

#[then("Alice owns 'name.meta' domain")]
fn alice_owns_the_domain(world: &mut ContractWorld) {
    let domain = world
        .state
        .pns
        .get_domain(string_to_bytes(DEFAULT_DOMAIN_NAME).as_slice())
        .unwrap();

    assert_eq!(
        world.state.nft.owner_of(domain.token_id),
        mock_address(ALICE_ADDRESS)
    );
}

#[then("Alice owns 'sub.name.meta' domain")]
fn alice_owns_the_subdomain(world: &mut ContractWorld) {
    let domain = world
        .state
        .pns
        .get_domain(string_to_bytes(DEFAULT_SUBDOMAIN_NAME).as_slice())
        .unwrap();

    assert_eq!(
        world.state.nft.owner_of(domain.token_id),
        mock_address(ALICE_ADDRESS)
    );
}

#[then("'sub.name.meta' domain is not minted")]
fn subdomain_is_not_minted(world: &mut ContractWorld) {
    let domain = world
        .state
        .pns
        .get_domain(string_to_bytes(DEFAULT_SUBDOMAIN_NAME).as_slice());

    assert_eq!(domain, None);
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(ContractWorld::run("tests/features"));
}
