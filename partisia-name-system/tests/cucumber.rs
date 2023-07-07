use std::panic::catch_unwind;

use cucumber::{given, then, when, World};
use partisia_name_system::{
    actions::{
        execute_init, execute_mint, execute_record_delete, execute_record_mint,
        execute_record_update,
    },
    msg::{PnsMintMsg, PnsRecordDeleteMsg, PnsRecordMintMsg, PnsRecordUpdateMsg},
    state::{PartisiaNameSystemState, RecordClass},
};
use utils::tests::{mock_contract_context, string_to_bytes};

fn get_record_class_given(class: String) -> RecordClass {
    match class.as_str() {
        "Wallet" => RecordClass::Wallet {},
        "Uri" => RecordClass::Uri {},
        "Twitter" => RecordClass::Twitter {},
        _ => panic!("Unknown record class"),
    }
}

#[derive(Debug, Default, World)]
pub struct PartisiaNameSystemWorld {
    state: PartisiaNameSystemState,
}

#[given("a PNS contract")]
fn pns_contract(world: &mut PartisiaNameSystemWorld) {
    let state = execute_init(&mock_contract_context(1));

    world.state = state;
}

#[given(regex = ".+ minted '(.+)' domain without a parent")]
#[when(regex = ".+ mints '(.+)' domain without a parent")]
fn mint_a_domain(world: &mut PartisiaNameSystemWorld, domain: String) {
    let msg = PnsMintMsg {
        domain: domain.into_bytes(),
        token_id: 0,
        parent_id: None,
    };

    let res = catch_unwind(|| {
        let mut state = world.state.clone();
        execute_mint(&mock_contract_context(1), &mut state, &msg);
        state
    });

    if let Ok(new_state) = res {
        world.state = new_state;
    }
}

#[given(regex = ".+ minted '(.+)' domain with '(.+)' domain as the parent")]
#[when(regex = ".+ mints '(.+)' domain with '(.+)' domain as the parent")]
fn mint_a_domain_with_parent(world: &mut PartisiaNameSystemWorld, domain: String, parent: String) {
    let msg = PnsMintMsg {
        domain: domain.into_bytes(),
        token_id: 0,
        parent_id: Some(parent.into_bytes()),
    };

    let res = catch_unwind(|| {
        let mut state = world.state.clone();
        execute_mint(&mock_contract_context(1), &mut state, &msg);
        state
    });

    if let Ok(new_state) = res {
        world.state = new_state;
    }
}

#[given(regex = ".+ (minted) the '(.+)' record with '(.+)' data for the '(.+)' domain")]
#[when(regex = ".+ (mints|updates) the '(.+)' record with '(.+)' data for the '(.+)' domain")]
fn mint_a_record(
    world: &mut PartisiaNameSystemWorld,
    action: String,
    class: String,
    data: String,
    domain: String,
) {
    let res = catch_unwind(|| {
        let mut state = world.state.clone();
        let context = &mock_contract_context(1);
        match action.as_str() {
            "mints" | "minted" => {
                let msg = PnsRecordMintMsg {
                    domain: domain.into_bytes(),
                    class: get_record_class_given(class),
                    data: data.into_bytes(),
                };
                execute_record_mint(context, &mut state, &msg);
            }

            "updates" => {
                let msg = PnsRecordUpdateMsg {
                    domain: domain.into_bytes(),
                    class: get_record_class_given(class),
                    data: data.into_bytes(),
                };

                execute_record_update(context, &mut state, &msg);
            }

            _ => panic!("Not handled"),
        };

        state
    });

    if let Ok(new_state) = res {
        world.state = new_state;
    }
}

// Alice deletes the 'Wallet' record for the 'meta.name' domain
#[when(regex = ".+ deletes the '(.+)' record for the '(.+)' domain")]
fn domain_record_delete(world: &mut PartisiaNameSystemWorld, class: String, domain: String) {
    let msg = PnsRecordDeleteMsg {
        domain: domain.into_bytes(),
        class: get_record_class_given(class),
    };

    execute_record_delete(&mock_contract_context(1), &mut world.state, &msg);
}

#[then(regex = "'(.+)' domain (is|is not) minted")]
fn is_domain_minted(world: &mut PartisiaNameSystemWorld, domain: String, action: String) {
    let domain = world.state.get_domain(string_to_bytes(&domain).as_slice());

    match action.as_str() {
        "is" => assert!(domain.is_some()),
        "is not" => assert!(domain.is_none()),
        _ => panic!("Not handled"),
    }
}

#[then(expr = "'{word}' domain has a '{word}' record with '{word}' data")]
fn domain_has_record(
    world: &mut PartisiaNameSystemWorld,
    domain: String,
    class: String,
    data: String,
) {
    let domain = world.state.get_domain(string_to_bytes(&domain).as_slice());

    if let Some(domain) = domain {
        let record = domain.get_record(&get_record_class_given(class)).unwrap();

        assert_eq!(record.data, data.into_bytes());
    }
}

#[then(expr = "'{word}' domain does not exist")]
fn has_no_domain(world: &mut PartisiaNameSystemWorld, domain: String) {
    let domain = world.state.get_domain(string_to_bytes(&domain).as_slice());

    assert_eq!(domain, None);
}

#[then(expr = "'{word}' domain does not have a '{word}' record")]
fn domain_has_no_record(world: &mut PartisiaNameSystemWorld, domain: String, class: String) {
    let domain = world.state.get_domain(string_to_bytes(&domain).as_slice());

    if let Some(domain) = domain {
        let record = domain.get_record(&get_record_class_given(class));

        assert_eq!(record, None);
    }
}

fn main() {
    futures::executor::block_on(PartisiaNameSystemWorld::run("tests/features"));
}
