use std::mem::take;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use meta_names_contract::{
    contract::{initialize, on_mint_callback, transfer_from},
    msg::{InitMsg, MintMsg},
    state::{ContractConfig, ContractState, Fees, PaymentInfo},
};
use utils::tests::{
    get_address_for_user, mock_address, mock_contract_context, mock_successful_callback_context,
    ALICE_ADDRESS, PAYMENT_TOKEN_ADDRESS, SYSTEM_ADDRESS,
};

fn setup_contract() -> ContractState {
    let config = ContractConfig {
        contract_enabled: true,
        payment_info: vec![PaymentInfo {
            id: 0,
            token: Some(mock_address(PAYMENT_TOKEN_ADDRESS)),
            receiver: Some(mock_address(ALICE_ADDRESS)),
            fees: Fees {
                mapping: vec![],
                default_fee: 1,
                decimals: 0,
            },
        }],
        ..ContractConfig::default()
    };

    let msg = InitMsg {
        admin_addresses: vec![mock_address(SYSTEM_ADDRESS)],
        config,
        name: "Meta Names".to_string(),
        symbol: "mpc".to_string(),
        uri_template: "metanames.io".to_string(),
    };

    let cxt = mock_contract_context(ALICE_ADDRESS);
    let (state, _) = initialize(cxt, msg);

    state
}

fn mint_domain(
    state: &mut ContractState,
    user: String,
    domain: String,
    payment_coin_id: u64,
) -> ContractState {
    let state = take(state);
    let (new_state, _) = on_mint_callback(
        mock_contract_context(get_address_for_user(user.clone())),
        mock_successful_callback_context(),
        state,
        MintMsg {
            domain,
            to: mock_address(get_address_for_user(user)),
            payment_coin_id,
            token_uri: None,
            parent_id: None,
            subscription_years: None,
        },
    );
    new_state
}

fn domain_transfer_from(
    state: &mut ContractState,
    user: String,
    token_id: u128,
    to: String,
) -> ContractState {
    let state = take(state);
    let (new_state, _) = transfer_from(
        mock_contract_context(get_address_for_user(user.clone())),
        state,
        mock_address(get_address_for_user(user.clone())),
        mock_address(get_address_for_user(to)),
        token_id,
    );

    new_state
}

fn benchmark_domain_transfer(domains_count: u64) {
    let mut state = setup_contract();
    let user = "alice".to_string();
    let domain = "test".to_string();
    let to = "bob".to_string();

    for i in 0..domains_count {
        let new_domain = format!("test{}", i);
        state = mint_domain(&mut state, user.clone(), new_domain, 0);
    }

    let mut state = mint_domain(&mut state, user.clone(), domain.clone(), 0);
    domain_transfer_from(&mut state, user, 0, to);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("domain transfer", |b| {
        b.iter(|| benchmark_domain_transfer(black_box(0)))
    });
    c.bench_function("domain transfer with 1k", |b| {
        b.iter(|| benchmark_domain_transfer(black_box(1000)))
    });
    c.bench_function("domain transfer with 10k", |b| {
        b.iter(|| benchmark_domain_transfer(black_box(10_000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
