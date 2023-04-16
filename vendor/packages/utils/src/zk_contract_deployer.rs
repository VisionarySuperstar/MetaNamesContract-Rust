use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    context::ContractContext,
    events::EventGroupBuilder,
};
use pbc_traits::ReadWriteRPC;

use crate::contract_deployer::init_msg_signature;

// Contract Deployer address
pub const ZK_CONTRACT_DEPLOYER: Address = Address {
    address_type: AddressType::SystemContract,
    identifier: [
        0x8b, 0xc1, 0xcc, 0xbb, 0x67, 0x2b, 0x87, 0x71, 0x03, 0x27, 0x71, 0x3c, 0x97, 0xd4, 0x32,
        0x04, 0x90, 0x50, 0x82, 0xcb,
    ],
};

pub const MIN_MPC_STAKE: u64 = 2000_0000;

/// ## Description
/// Creates event that will deploy a new zero-knowledge contract.
/// Returns newly deployed contract address
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **event_group** is an object of type [`EventGroupBuilder`]
///
/// * **zkwa** is an object of type [`&[u8]`]
///
/// * **abi** is an object of type [`&[u8]`]
///
/// * **init_msg** is an object of type [`T`]
///
/// * **mpc_token_stake** is an optional variable of type [`u64`]
pub fn add_zk_contract_deploy_event_with_msg<T>(
    ctx: &ContractContext,
    event_group: &mut EventGroupBuilder,
    zkwa: &[u8],
    abi: &[u8],
    init_msg: &T,
    mpc_token_stake: Option<u64>,
) -> Address
where
    T: ReadWriteRPC,
{
    let mut raw_init_msg: Vec<u8> = vec![];
    init_msg.rpc_write_to(&mut raw_init_msg).unwrap();

    add_zk_contract_deploy_event(
        ctx,
        event_group,
        zkwa,
        abi,
        &raw_init_msg,
        mpc_token_stake.unwrap_or(MIN_MPC_STAKE),
    )
}

/// ## Description
/// Creates event that will deploy a new zero-knowledge contract.
/// Returns newly deployed contract address
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **event_group** is an object of type [`EventGroupBuilder`]
///
/// * **zkwa** is an object of type [`&[u8]`]
///
/// * **abi** is an object of type [`&[u8]`]
///
/// * **init_msg** is an object of type [`&[u8]`]
///
/// * **mpc_token_stake** is an optional variable of type [`u64`]
pub fn add_zk_contract_deploy_event(
    ctx: &ContractContext,
    event_group: &mut EventGroupBuilder,
    zkwa: &[u8],
    abi: &[u8],
    init_msg: &[u8],
    mpc_token_stake: u64,
) -> Address {
    let mut msg: Vec<u8> = init_msg_signature();
    msg.extend(init_msg);

    event_group
        .call(ZK_CONTRACT_DEPLOYER, Shortname::from_u32(0x00))
        .argument(zkwa.to_vec())
        .argument(msg.to_vec())
        .argument(abi.to_vec())
        .argument(mpc_token_stake)
        .done();

    Address {
        address_type: AddressType::ZkContract,
        identifier: ctx.original_transaction[12..32].try_into().unwrap(),
    }
}
