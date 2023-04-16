use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    context::ContractContext,
    events::EventGroupBuilder,
};
use pbc_traits::ReadWriteRPC;

// Contract Deployer address
pub const CONTRACT_DEPLOYER: Address = Address {
    address_type: AddressType::SystemContract,
    identifier: [
        0x97, 0xa0, 0xe2, 0x38, 0xe9, 0x24, 0x02, 0x5b, 0xad, 0x14, 0x4a, 0xa0, 0xc4, 0x91, 0x3e,
        0x46, 0x30, 0x8f, 0x9a, 0x4d,
    ],
};

/// ## Description
/// Creates event that will deploy a new contract.
/// Returns newly deployed contract address
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **event_group** is an object of type [`EventGroupBuilder`]
///
/// * **wasm** is an object of type [`&[u8]`]
///
/// * **abi** is an object of type [`&[u8]`]
///
/// * **init_msg** is an object of type [`T`]
pub fn add_contract_deploy_event_with_msg<T>(
    ctx: &ContractContext,
    event_group: &mut EventGroupBuilder,
    wasm: &[u8],
    abi: &[u8],
    init_msg: &T,
) -> Address
where
    T: ReadWriteRPC,
{
    let mut raw_init_msg: Vec<u8> = vec![];
    init_msg.rpc_write_to(&mut raw_init_msg).unwrap();

    add_contract_deploy_event(ctx, event_group, wasm, abi, &raw_init_msg)
}

/// ## Description
/// Creates event that will deploy a new contract.
/// Returns newly deployed contract address
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **event_group** is an object of type [`EventGroupBuilder`]
///
/// * **wasm** is an object of type [`&[u8]`]
///
/// * **abi** is an object of type [`&[u8]`]
///
/// * **init_msg** is an object of type [`&[u8]`]
pub fn add_contract_deploy_event(
    ctx: &ContractContext,
    event_group: &mut EventGroupBuilder,
    wasm: &[u8],
    abi: &[u8],
    init_msg: &[u8],
) -> Address {
    let mut msg: Vec<u8> = init_msg_signature();
    msg.extend(init_msg);

    event_group
        .call(CONTRACT_DEPLOYER, Shortname::from_u32(1))
        .argument(wasm.to_vec())
        .argument(abi.to_vec())
        .argument(msg.to_vec())
        .done();

    Address {
        address_type: AddressType::PublicContract,
        identifier: ctx.original_transaction[12..32].try_into().unwrap(),
    }
}

#[inline]
pub fn init_msg_signature() -> Vec<u8> {
    vec![0xff, 0xff, 0xff, 0xff, 0x0f]
}
