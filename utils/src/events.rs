use pbc_contract_common::{
    address::{Address, ShortnameCallback},
    context::CallbackContext,
    events::EventGroupBuilder,
};
use pbc_traits::ReadRPC;
use pbc_traits::WriteRPC;

/// This trait describes methods that must be implemented
/// in order to be able to convert a struct into rpc event
pub trait IntoShortnameRPCEvent {
    fn action_shortname(&self) -> u32;
    fn as_interaction(&self, builder: &mut EventGroupBuilder, dest: &Address);
}

/// This trait describes methods that must be implemented
/// in order to be able to convert a struct into rpc event with specified cost
pub trait IntoShortnameRPCEventWithCost {
    fn action_shortname(&self) -> u32;
    fn as_interaction(&self, builder: &mut EventGroupBuilder, dest: &Address, cost: u64);
}

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
    T: ReadRPC + WriteRPC + Clone,
{
    builder
        .with_callback(ShortnameCallback::from_u32(callback_byte))
        .argument(msg.clone())
        .done();
}

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
    T: ReadRPC + WriteRPC + Clone,
{
    builder
        .with_callback(ShortnameCallback::from_u32(callback_byte))
        .argument(msg.clone())
        .with_cost(cost)
        .done();
}

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
    use pbc_contract_common::address::{Address, Shortname};
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
                .argument(self.to)
                .argument(self.amount)
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
}

#[cfg(test)]
mod rpc_msg_with_cost_tests {
    use super::IntoShortnameRPCEventWithCost;

    use create_type_spec_derive::CreateTypeSpec;
    use pbc_contract_common::address::{Address, Shortname};
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
                .argument(self.to)
                .argument(self.amount)
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
}
