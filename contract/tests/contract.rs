use cucumber::{given, then, when, World};
use meta_names_contract::{
    contract::{initialize, mint},
    msg::InitMsg,
    state::ContractState,
};
use utils::tests::{mock_address, mock_contract_context, string_to_bytes};

const DEFAULT_DOMAIN_NAME: &str = "name.meta";

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

#[when("Alice mints a domain without a parent")]
fn mint_a_domain(world: &mut ContractWorld) {
    let (new_state, _) = mint(
        mock_contract_context(2),
        world.state.clone(),
        string_to_bytes(DEFAULT_DOMAIN_NAME),
        mock_address(1u8),
        None,
        None,
    );

    world.state = new_state;
}

#[then("Alice owns the domain")]
fn alice_owns_the_domain(world: &mut ContractWorld) {
    let domain = world
        .state
        .pns
        .get_domain(string_to_bytes(DEFAULT_DOMAIN_NAME).as_slice())
        .unwrap();

    assert_eq!(world.state.nft.owner_of(domain.token_id), mock_address(1u8));
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(ContractWorld::run("tests/features"));
}
