use std::panic::catch_unwind;

use cucumber::{given, then, when, World};
use meta_names_contract::{
    contract::{
        approve_domain, initialize, mint, on_mint_callback, update_admin_address, ADMIN_ROLE,
    },
    msg::{InitMsg, MintMsg},
    state::{ContractState, PayableMintInfo},
};
use utils::tests::{mock_address, mock_contract_context, mock_successful_callback_context};

const SYSTEM_ADDRESS: u8 = 0;
const ALICE_ADDRESS: u8 = 1;
const BOB_ADDRESS: u8 = 2;
const PAYABLE_TOKEN_ADDRESS: u8 = 10;

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
        admin_addresses: vec![mock_address(SYSTEM_ADDRESS)],
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        uri_template: "metanames.io".to_string(),
        payable_mint_info: PayableMintInfo {
            token: Some(mock_address(PAYABLE_TOKEN_ADDRESS)),
            receiver: Some(mock_address(ALICE_ADDRESS)),
        },
    };

    let (state, _) = initialize(mock_contract_context(ALICE_ADDRESS), msg);
    world.state = state;
}

#[given(expr = "{word} minted '{word}' domain without a parent")]
#[when(expr = "{word} mints '{word}' domain without fees and a parent")]
fn mint_a_domain(world: &mut ContractWorld, user: String, domain: String) {
    let res = catch_unwind(|| {
        on_mint_callback(
            mock_contract_context(get_address_for_user(user.clone())),
            mock_successful_callback_context(),
            world.state.clone(),
            MintMsg {
                domain,
                to: mock_address(get_address_for_user(user)),
                token_uri: None,
                parent_id: None,
            },
        )
    });

    if let Ok((new_state, _)) = res {
        world.state = new_state;
    }
}

#[given(regex = r"(\w+) user (with) the admin (role)")]
#[when(regex = r"(\w+) user (grants|denies) the admin role for (\w+) user")]
fn user_admin_role(world: &mut ContractWorld, admin: String, action: String, user: String) {
    let res = catch_unwind(|| match action.as_str() {
        "with" => update_admin_address(
            mock_contract_context(SYSTEM_ADDRESS),
            world.state.clone(),
            mock_address(get_address_for_user(admin)),
            true,
        ),
        "grants" | "denied" => update_admin_address(
            mock_contract_context(get_address_for_user(admin)),
            world.state.clone(),
            mock_address(get_address_for_user(user)),
            action == "grants",
        ),
        _ => panic!("Unknown action"),
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
#[when(regex = r"(\w+) mints '(.+)' domain without (a parent)")]
fn mint_domain_with_parent(
    world: &mut ContractWorld,
    user: String,
    domain: String,
    parent: String,
) {
    let res = catch_unwind(|| {
        let parent_opt = if parent == "a parent" {
            None
        } else {
            Some(parent.clone())
        };

        mint(
            mock_contract_context(get_address_for_user(user.clone())),
            world.state.clone(),
            domain,
            mock_address(get_address_for_user(user)),
            None,
            parent_opt,
        )
    });

    if let Ok((new_state, _)) = res {
        world.state = new_state;
    }
}

#[then(expr = "{word} owns '{word}' domain")]
fn owns_the_domain(world: &mut ContractWorld, user: String, domain: String) {
    let domain = world.state.pns.get_domain(&domain).unwrap();

    assert_eq!(
        world.state.nft.owner_of(domain.token_id),
        mock_address(get_address_for_user(user))
    );
}

#[then(expr = "'{word}' domain is not minted")]
fn domain_is_not_minted(world: &mut ContractWorld, domain: String) {
    let domain = world.state.pns.get_domain(&domain);

    assert_eq!(domain, None);
}

#[then(regex = r"(\w+) user (is|is not) an admin")]
fn user_is_admin(world: &mut ContractWorld, user: String, is: String) {
    let is_admin = world
        .state
        .access_control
        .has_role(ADMIN_ROLE, &mock_address(get_address_for_user(user)));

    assert_eq!(is_admin, is == "is");
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(ContractWorld::run("tests/features"));
}
