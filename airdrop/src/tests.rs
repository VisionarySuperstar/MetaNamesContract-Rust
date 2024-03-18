// Setup tests

use utils::tests::mock_address;

use crate::actions::{execute_add_airdrop, execute_airdrop, execute_init};

#[test]
fn proper_has_airdrop() {
    let address = mock_address(0);
    let mut state = execute_init();

    assert_eq!(state.has_airdrop(&address), false);

    execute_add_airdrop(&mut state, &address);

    assert_eq!(state.has_airdrop(&address), true);
}

#[test]
fn proper_execute_airdrop() {
    let address = mock_address(0);
    let mut state = execute_init();

    execute_add_airdrop(&mut state, &address);
    execute_airdrop(&mut state, &address);

    assert_eq!(state.has_airdrop(&address), false);
}

#[test]
fn proper_execute_add_airdrop() {
    let address = mock_address(0);
    let mut state = execute_init();

    execute_add_airdrop(&mut state, &address);
    execute_add_airdrop(&mut state, &address);

    assert_eq!(state.has_airdrop(&address), true);
    assert_eq!(state.inventory.get(&address).unwrap(), 2);
}

#[test]
#[should_panic(expected = "No airdrop for the address")]
fn proper_execute_airdrop_no_airdrop() {
    let address = mock_address(0);
    let mut state = execute_init();

    execute_airdrop(&mut state, &address);
}
