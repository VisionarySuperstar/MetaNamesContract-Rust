use pbc_contract_common::{
    address::{Address, ShortnameCallback},
    context::CallbackContext,
    events::EventGroupBuilder,
};
use pbc_traits::ReadWriteRPC;

/// ## Description
/// This trait describes methods that must be implemented
/// in order to be able to convert a struct into rpc event
pub trait IntoShortnameRPCEvent {
    fn action_shortname(&self) -> u32;
    fn as_interaction(&self, builder: &mut EventGroupBuilder, dest: &Address);
}

/// ## Description
/// This trait describes methods that must be implemented
/// in order to be able to convert a struct into rpc event with specified cost
pub trait IntoShortnameRPCEventWithCost {
    fn action_shortname(&self) -> u32;
    fn as_interaction(&self, builder: &mut EventGroupBuilder, dest: &Address, cost: u64);
}

/// ## Description
/// Creates a callback event and adds it to event group builder object
/// ## Params
/// * **builder** is an object of type [`EventGroupBuilder`]
///
/// * **callback_byte** is an object of type [`u32`]
///
/// * **msg** is an object of type [`T`]
#[inline]
pub fn build_msg_callback<T>(builder: &mut EventGroupBuilder, callback_byte: u32, msg: &T)
where
    T: ReadWriteRPC + Clone,
{
    builder
        .with_callback(ShortnameCallback::from_u32(callback_byte))
        .argument(msg.clone())
        .done();
}

/// ## Description
/// Creates a callback event with specified cost and adds it to event group builder object
/// ## Params
/// * **builder** is an object of type [`EventGroupBuilder`]
///
/// * **callback_byte** is an object of type [`u32`]
///
/// * **msg** is an object of type [`T`]
///
/// * **cost** is an object of type [`u64`]
#[inline]
pub fn build_msg_callback_with_cost<T>(
    builder: &mut EventGroupBuilder,
    callback_byte: u32,
    msg: &T,
    cost: u64,
) where
    T: ReadWriteRPC + Clone,
{
    builder
        .with_callback(ShortnameCallback::from_u32(callback_byte))
        .argument(msg.clone())
        .with_cost(cost)
        .done();
}

/// ## Description
/// Validates that all spawned events from original action was executed successfully
/// ## Params
/// * **callback_ctx** is an object of type [`CallbackContext`]
#[inline]
pub fn assert_callback_success(callback_ctx: &CallbackContext) {
    assert!(
        callback_ctx.success && callback_ctx.results.iter().all(|res| res.succeeded),
        "Callback has errors"
    );
}

#[cfg(test)]
mod rpc_msg_tests {
    use super::*;

    use create_type_spec_derive::CreateTypeSpec;
    use pbc_contract_common::{
        address::{Address, AddressType, Shortname},
        events::EventGroup,
    };
    use read_write_rpc_derive::ReadWriteRPC;
    use rpc_msg_derive::IntoShortnameRPCEvent;

    #[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
    pub struct TestTransferMsg {
        pub to: Address,
        pub amount: u128,
        pub memo: String,
        pub amounts: Vec<u128>,
    }

    impl IntoShortnameRPCEvent for TestTransferMsg {
        fn action_shortname(&self) -> u32 {
            0x01
        }

        fn as_interaction(
            &self,
            builder: &mut pbc_contract_common::events::EventGroupBuilder,
            dest: &Address,
        ) {
            builder
                .call(*dest, Shortname::from_u32(self.action_shortname()))
                .argument(self.to.clone())
                .argument(self.amount.clone())
                .argument(self.memo.clone())
                .argument(self.amounts.clone())
                .done();
        }
    }

    #[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
    #[rpc_msg(action = 0x01)]
    pub struct TestTransferMsgDerive {
        pub to: Address,
        pub amount: u128,
        pub memo: String,
        pub amounts: Vec<u128>,
    }

    #[test]
    fn test_derive_macro() {
        fn mock_address(le: u8) -> Address {
            Address {
                address_type: AddressType::Account,
                identifier: [
                    le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8,
                ],
            }
        }

        let msg = TestTransferMsg {
            to: mock_address(1u8),
            amount: 100,
            memo: "memo".to_string(),
            amounts: vec![10],
        };

        let derive_msg = TestTransferMsgDerive {
            to: msg.to.clone(),
            amount: msg.amount,
            memo: msg.memo.clone(),
            amounts: msg.amounts.clone(),
        };

        assert_eq!(msg.action_shortname(), derive_msg.action_shortname());

        let dest = mock_address(10u8);
        let mut eg = EventGroup::builder();
        msg.as_interaction(&mut eg, &dest);

        let mut derive_eg = EventGroup::builder();
        derive_msg.as_interaction(&mut derive_eg, &dest);

        assert_eq!(eg.build(), derive_eg.build());
    }
}

#[cfg(test)]
mod rpc_msg_with_cost_tests {
    use super::IntoShortnameRPCEventWithCost;

    use create_type_spec_derive::CreateTypeSpec;
    use pbc_contract_common::{
        address::{Address, AddressType, Shortname},
        events::EventGroup,
    };
    use read_write_rpc_derive::ReadWriteRPC;
    use rpc_msg_derive::IntoShortnameRPCEventWithCost;

    #[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
    pub struct TestTransferMsgWithCost {
        pub to: Address,
        pub amount: u128,
        pub memo: String,
        pub amounts: Vec<u128>,
    }

    impl IntoShortnameRPCEventWithCost for TestTransferMsgWithCost {
        fn action_shortname(&self) -> u32 {
            0x01
        }

        fn as_interaction(
            &self,
            builder: &mut pbc_contract_common::events::EventGroupBuilder,
            dest: &Address,
            cost: u64,
        ) {
            builder
                .call(*dest, Shortname::from_u32(self.action_shortname()))
                .with_cost(cost)
                .argument(self.to.clone())
                .argument(self.amount.clone())
                .argument(self.memo.clone())
                .argument(self.amounts.clone())
                .done();
        }
    }

    #[derive(
        ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEventWithCost, Clone, PartialEq, Eq, Debug,
    )]
    #[rpc_msg(action = 0x01)]
    pub struct TestTransferMsgWithCostDerive {
        pub to: Address,
        pub amount: u128,
        pub memo: String,
        pub amounts: Vec<u128>,
    }

    #[test]
    fn test_derive_macro() {
        fn mock_address(le: u8) -> Address {
            Address {
                address_type: AddressType::Account,
                identifier: [
                    le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                    0u8, 0u8, 0u8, 0u8,
                ],
            }
        }

        let msg = TestTransferMsgWithCost {
            to: mock_address(1u8),
            amount: 100,
            memo: "memo".to_string(),
            amounts: vec![10],
        };

        let derive_msg = TestTransferMsgWithCostDerive {
            to: msg.to.clone(),
            amount: msg.amount,
            memo: msg.memo.clone(),
            amounts: msg.amounts.clone(),
        };

        assert_eq!(msg.action_shortname(), derive_msg.action_shortname());

        let dest = mock_address(10u8);
        let mut eg = EventGroup::builder();
        msg.as_interaction(&mut eg, &dest, 100);

        let mut derive_eg = EventGroup::builder();
        derive_msg.as_interaction(&mut derive_eg, &dest, 100);

        assert_eq!(eg.build(), derive_eg.build());
    }
}
