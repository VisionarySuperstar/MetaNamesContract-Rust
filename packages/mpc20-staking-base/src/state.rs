use rust_decimal::prelude::*;
use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use mpc20_base::state::MPC20ContractState;
use utils::decimal::DecimalRatio;

/// ## Description
/// This structure describes main mpc20-staking contract state.
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MPC20StakingContractState {
    /// deposit token address
    pub deposit_token: Address,
    /// per epoch distribution amount
    pub distribution_amount: u128,
    /// UTC timestamp
    pub distribution_epoch: u64,

    /// global index for calculating users share
    pub global_index: DecimalRatio,
    /// total amount of tokens staked
    pub total_staked: u128,
    /// UTC timestamp of last distribution
    pub last_distributed: u64,

    /// information about stakers
    pub stakers: BTreeMap<Address, Staker>,
    /// compounding limit
    pub compound_frequency: u64,
    /// mpc20 base state
    pub mpc20: MPC20ContractState,
}

impl MPC20StakingContractState {
    /// ## Description
    /// Distributes rewards by recalculting global index
    /// ## Params
    /// * **block_time** is an object of type [`u64`]
    pub fn distribute_rewards(&mut self, block_time: u64) {
        if self.total_staked.is_zero() {
            self.last_distributed = block_time;
            return;
        }

        let passed_distributions = (block_time - self.last_distributed) / self.distribution_epoch;
        if passed_distributions.is_zero() {
            return;
        }

        let distributed_amount = self.distribution_amount * (passed_distributions as u128);
        self.global_index =
            self.global_index + DecimalRatio::from_ratio(distributed_amount, self.total_staked);
        self.last_distributed += self.distribution_epoch * passed_distributions;
    }

    /// ## Description
    /// Increases total staked amount and staked amount by staker
    /// ## Params
    /// * **address** is an object of type [`Address`]
    ///
    /// * **staker** is an object of type [`Staker`]
    ///
    /// * **amount** is a field of type [`u128`]
    pub fn increase_stake_amount(&mut self, address: &Address, staker: &mut Staker, amount: u128) {
        self.total_staked = self.total_staked.checked_add(amount).unwrap();
        staker.staked_amount = staker.staked_amount.checked_add(amount).unwrap();
        self.store_staker(address, staker);
    }

    /// ## Description
    /// Decreases total staked amount and staked amount by staker
    /// ## Params
    /// * **address** is an object of type [`Address`]
    ///
    /// * **staker** is an object of type [`Staker`]
    ///
    /// * **amount** is a field of type [`u128`]
    pub fn decrease_stake_amount(&mut self, address: &Address, staker: &mut Staker, amount: u128) {
        self.total_staked = self.total_staked.checked_sub(amount).unwrap();
        staker.staked_amount = staker.staked_amount.checked_sub(amount).unwrap();
        self.store_staker(address, staker);
    }

    /// ## Description
    /// Saves updated information about staker
    /// ## Params
    /// * **address** is an object of type [`Address`]
    ///
    /// * **staker** is an object of type [`Staker`]
    pub fn store_staker(&mut self, address: &Address, staker: &Staker) {
        self.stakers
            .entry(*address)
            .and_modify(|s| {
                s.reward_index = staker.reward_index;
                s.staked_amount = staker.staked_amount;
                s.pending_reward = staker.pending_reward;
                s.last_compound = staker.last_compound;
            })
            .or_insert_with(|| staker.clone());
    }

    /// ## Description
    /// Returns information about staker
    /// ## Params
    /// * **address** is an object of type [`Address`]
    pub fn get_staker(&self, address: &Address) -> Staker {
        match self.stakers.get(address) {
            Some(s) => s.clone(),
            None => Staker {
                reward_index: DecimalRatio::default(),
                staked_amount: 0,
                pending_reward: 0,
                last_compound: 0,
            },
        }
    }
}

/// ## Description
/// This structure describes information about staker
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Staker {
    /// personal reward index
    pub reward_index: DecimalRatio,
    /// total staked amount
    pub staked_amount: u128,
    /// pending rewards
    pub pending_reward: u128,
    /// UTC timestamp of last compounding
    pub last_compound: u64,
}

impl Staker {
    /// ## Description
    /// Computes current staker reward
    /// ## Params
    /// * **global_index** is an object of type [`DecimalRatio`]
    pub fn compute_reward(&mut self, global_index: DecimalRatio) {
        let staked_amount = DecimalRatio::new(self.staked_amount, 0);
        let pending_reward = (staked_amount * global_index) - (staked_amount * self.reward_index);

        self.reward_index = global_index;
        self.pending_reward = self
            .pending_reward
            .checked_add(pending_reward.to_u128())
            .unwrap();
    }
}
