# Graph Network Subgraph fed by Substreams

Substreams based Graph Network subgraph and substreams. 

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Introduction 

This project is a [subgraph](https://thegraph.com/docs/en/developing/creating-a-subgraph/) fed by [substreams](https://substreams.streamingfast.io/) that allows you to obtain data for The Graph Network. 

## Features 

### Available Data 

This subgraph makes available the following data:
- Total supply, total mints and burns of GRT, 
- GRT balances of addresses
- In-protocol balances like indexer, delegator and curator stakes 

### Substreams Module Graph

Here is the graph of the modules of the substreams: 

```mermaid
graph LR;
  map_events[map: map_events]
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_events
  store_grt_balances[store: store_grt_balances]
  map_events --> store_grt_balances
  store_grt_global[store: store_grt_global]
  map_events --> store_grt_global
  store_indexer_stakes[store: store_indexer_stakes]
  map_events --> store_indexer_stakes
  store_graph_account_indexer[store: store_graph_account_indexer]
  map_events --> store_graph_account_indexer
  store_graph_account_delegator[store: store_graph_account_delegator]
  map_events --> store_graph_account_delegator
  store_graph_account_curator[store: store_graph_account_curator]
  map_events --> store_graph_account_curator
  store_cumulative_delegated_stakes[store: store_cumulative_delegated_stakes]
  map_events --> store_cumulative_delegated_stakes
  store_cumulative_delegator_stakes[store: store_cumulative_delegator_stakes]
  map_events --> store_cumulative_delegator_stakes
  store_total_delegated_stakes[store: store_total_delegated_stakes]
  map_events --> store_total_delegated_stakes
  store_delegation_parameters --> store_total_delegated_stakes
  store_delegation_parameters[store: store_delegation_parameters]
  map_events --> store_delegation_parameters
  store_total_signalled[store: store_total_signalled]
  map_events --> store_total_signalled
  store_cumulative_curator_signalled[store: store_cumulative_curator_signalled]
  map_events --> store_cumulative_curator_signalled
  store_cumulative_curator_burned[store: store_cumulative_curator_burned]
  map_events --> store_cumulative_curator_burned
  map_graph_network_entities[map: map_graph_network_entities]
  store_grt_global -- deltas --> map_graph_network_entities
  map_graph_account_entities[map: map_graph_account_entities]
  store_grt_balances -- deltas --> map_graph_account_entities
  store_graph_account_indexer -- deltas --> map_graph_account_entities
  store_graph_account_delegator -- deltas --> map_graph_account_entities
  store_graph_account_curator -- deltas --> map_graph_account_entities
  map_indexer_entities[map: map_indexer_entities]
  store_indexer_stakes -- deltas --> map_indexer_entities
  map_delegated_stake_entities[map: map_delegated_stake_entities]
  store_cumulative_delegated_stakes -- deltas --> map_delegated_stake_entities
  store_cumulative_delegator_stakes -- deltas --> map_delegated_stake_entities
  store_total_delegated_stakes -- deltas --> map_delegated_stake_entities
  map_curator_entities[map: map_curator_entities]
  store_cumulative_curator_signalled -- deltas --> map_curator_entities
  store_cumulative_curator_burned -- deltas --> map_curator_entities
  store_total_signalled -- deltas --> map_curator_entities
  graph_out[map: graph_out]
  map_graph_network_entities --> graph_out
  map_graph_account_entities --> graph_out
  map_indexer_entities --> graph_out
  map_delegated_stake_entities --> graph_out
  map_curator_entities --> graph_out
```


## Quickstart
To build and run the substream, 

1. [Install dependencies](https://substreams.streamingfast.io/developers-guide/installation-requirements).
2. [Get authentication](https://substreams.streamingfast.io/reference-and-specs/authentication).
3. Clone this repo
```console
git clone https://github.com/graphops/graph-network-substreams.git
```
4. Code gen with 
```console
substreams protogen ./substreams.yaml
``` 
5. Build the substream with 
```console
cargo build --target wasm32-unknown-unknown --release
``` 
6. Run the graph_out module with
```console
substreams run -e mainnet.eth.streamingfast.io:443 \
substreams.yaml \
graph_out \
--start-block 11446769
```

## Contributing

We welcome and appreciate your contributions! Please see the [Contributor Guide](/CONTRIBUTING.md), [Code Of Conduct](/CODE_OF_CONDUCT.md) and [Security Notes](/SECURITY.md) for this repository.
