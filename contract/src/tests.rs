use pbc_contract_common::{events::EventGroup, shortname::Shortname};
use utils::events::IntoShortnameRPCEvent;
use utils::tests::mock_address;

use crate::{actions::calculate_mint_fees, msg::MPC20TransferFromMsg};

#[test]
fn proper_mint_callback() {
    let dest = mock_address(30u8);

    let msg = MPC20TransferFromMsg {
        from: mock_address(10u8),
        to: mock_address(20u8),
        amount: 10,
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(0x03))
        .argument(mock_address(10u8))
        .argument(mock_address(20u8))
        .argument(10u128)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn test_mint_fees() {
    let fees_tuples = [
        ("n", 200),
        ("na", 150),
        ("nam", 100),
        ("name", 50),
        ("names", 5),
        ("verylongname", 5),
    ];

    for (name, fee) in fees_tuples {
        let fees = calculate_mint_fees(name, 1);
        assert_eq!(fees, fee);
    }
}
