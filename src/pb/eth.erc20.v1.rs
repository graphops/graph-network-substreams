// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Approvals {
    #[prost(message, repeated, tag="1")]
    pub approvals: ::prost::alloc::vec::Vec<Approval>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakeDepositedEvents {
    #[prost(message, repeated, tag="1")]
    pub stake_deposited_events: ::prost::alloc::vec::Vec<StakeDeposited>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakeWithdrawnEvents {
    #[prost(message, repeated, tag="1")]
    pub stake_withdrawn_events: ::prost::alloc::vec::Vec<StakeWithdrawn>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakeDelegatedEvents {
    #[prost(message, repeated, tag="1")]
    pub stake_delegated_events: ::prost::alloc::vec::Vec<StakeDelegated>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakeDelegatedLockedEvents {
    #[prost(message, repeated, tag="1")]
    pub stake_delegated_locked_events: ::prost::alloc::vec::Vec<StakeDelegatedLocked>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebateClaimedEvents {
    #[prost(message, repeated, tag="1")]
    pub rebate_claimed_events: ::prost::alloc::vec::Vec<RebateClaimed>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationParametersUpdatedEvents {
    #[prost(message, repeated, tag="1")]
    pub delegation_parameters_updated_events: ::prost::alloc::vec::Vec<DelegationParametersUpdated>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsAssignedEvents {
    #[prost(message, repeated, tag="1")]
    pub rewards_assigned_events: ::prost::alloc::vec::Vec<RewardsAssigned>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignalledEvents {
    #[prost(message, repeated, tag="1")]
    pub signalled_events: ::prost::alloc::vec::Vec<Signalled>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnedEvents {
    #[prost(message, repeated, tag="1")]
    pub burned_events: ::prost::alloc::vec::Vec<Burned>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationCreatedEvents {
    #[prost(message, repeated, tag="1")]
    pub allocation_created_events: ::prost::alloc::vec::Vec<AllocationCreated>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationClosedEvents {
    #[prost(message, repeated, tag="1")]
    pub allocation_closed_events: ::prost::alloc::vec::Vec<AllocationClosed>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationCollectedEvents {
    #[prost(message, repeated, tag="1")]
    pub allocation_collected_events: ::prost::alloc::vec::Vec<AllocationCollected>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexerStakes {
    #[prost(message, repeated, tag="1")]
    pub indexer_stakes: ::prost::alloc::vec::Vec<IndexerStake>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationPools {
    #[prost(message, repeated, tag="1")]
    pub delegation_pools: ::prost::alloc::vec::Vec<DelegationPool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorRewards {
    #[prost(message, repeated, tag="1")]
    pub delegator_rewards: ::prost::alloc::vec::Vec<DelegatorReward>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurationPools {
    #[prost(message, repeated, tag="1")]
    pub curation_pools: ::prost::alloc::vec::Vec<CurationPool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubgraphAllocations {
    #[prost(message, repeated, tag="1")]
    pub subgraph_allocations: ::prost::alloc::vec::Vec<SubgraphAllocation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexingRewards {
    #[prost(message, repeated, tag="1")]
    pub indexing_rewards: ::prost::alloc::vec::Vec<IndexingReward>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageChanges {
    #[prost(message, optional, tag="1")]
    pub indexer_stakes: ::core::option::Option<IndexerStakes>,
    #[prost(message, optional, tag="2")]
    pub delegation_pools: ::core::option::Option<DelegationPools>,
    #[prost(message, optional, tag="3")]
    pub delegator_rewards: ::core::option::Option<DelegatorRewards>,
    #[prost(message, optional, tag="4")]
    pub curation_pools: ::core::option::Option<CurationPools>,
    #[prost(message, optional, tag="5")]
    pub subgraph_allocations: ::core::option::Option<SubgraphAllocations>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, optional, tag="1")]
    pub transfers: ::core::option::Option<Transfers>,
    #[prost(message, optional, tag="2")]
    pub stake_deposited_events: ::core::option::Option<StakeDepositedEvents>,
    #[prost(message, optional, tag="3")]
    pub stake_withdrawn_events: ::core::option::Option<StakeWithdrawnEvents>,
    #[prost(message, optional, tag="4")]
    pub stake_delegated_events: ::core::option::Option<StakeDelegatedEvents>,
    #[prost(message, optional, tag="5")]
    pub stake_delegated_locked_events: ::core::option::Option<StakeDelegatedLockedEvents>,
    #[prost(message, optional, tag="6")]
    pub rebate_claimed_events: ::core::option::Option<RebateClaimedEvents>,
    #[prost(message, optional, tag="7")]
    pub rewards_assigned_events: ::core::option::Option<RewardsAssignedEvents>,
    #[prost(message, optional, tag="8")]
    pub delegation_parameters_updated_events: ::core::option::Option<DelegationParametersUpdatedEvents>,
    #[prost(message, optional, tag="9")]
    pub signalled_events: ::core::option::Option<SignalledEvents>,
    #[prost(message, optional, tag="10")]
    pub burned_events: ::core::option::Option<BurnedEvents>,
    #[prost(message, optional, tag="11")]
    pub allocation_created_events: ::core::option::Option<AllocationCreatedEvents>,
    #[prost(message, optional, tag="12")]
    pub allocation_closed_events: ::core::option::Option<AllocationClosedEvents>,
    #[prost(message, optional, tag="13")]
    pub allocation_collected_events: ::core::option::Option<AllocationCollectedEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub value: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Approval {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub spender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakeDeposited {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub indexer: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub tokens: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakeWithdrawn {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub indexer: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub tokens: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakeDelegated {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub indexer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub delegator: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub tokens: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StakeDelegatedLocked {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub indexer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub delegator: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub tokens: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebateClaimed {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub indexer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub subgraph_deployment_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub allocation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub tokens: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub delegation_fees: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationParametersUpdated {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub indexer: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub indexing_reward_cut: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub query_fee_cut: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub delegator_parameter_cooldown: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub block_number: u64,
    #[prost(uint64, tag="7")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsAssigned {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub indexer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub allocation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub epoch: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signalled {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub curator: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub subgraph_deployment_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub tokens: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub signal: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub curation_tax: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Burned {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub curator: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub subgraph_deployment_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub tokens: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub signal: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationCreated {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub indexer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub creator: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub active_for: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub subgraph_deployment_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub created_at_epoch: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub created_at_block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub created_at_block_number: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub tokens: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="11")]
    pub allocation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="12")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationClosed {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub indexer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub subgraph_deployment_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub closed_at_epoch: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub closed_at_block_hash: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub closed_at_block_number: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub closed_at: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub tokens: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub allocation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="10")]
    pub effective_allocation: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub poi: ::prost::alloc::string::String,
    #[prost(uint64, tag="12")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationCollected {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub indexer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub subgraph_deployment_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub epoch: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub tokens: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub allocation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub curation_fees: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub rebate_fees: ::prost::alloc::string::String,
    #[prost(uint64, tag="9")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexerStake {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub indexer: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub new_stake: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub old_stake: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationPool {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub indexer: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub new_stake: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub old_stake: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorReward {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub allocation_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub rewards: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurationPool {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub subgraph_deployment_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_signal: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub old_signal: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubgraphAllocation {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub subgraph_deployment_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_tokens: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub old_tokens: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexingReward {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub indexer: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub subgraph_deployment_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub allocation_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub indexer_rewards: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub delegator_rewards: ::prost::alloc::string::String,
    #[prost(uint64, tag="8")]
    pub ordinal: u64,
}
// @@protoc_insertion_point(module)
