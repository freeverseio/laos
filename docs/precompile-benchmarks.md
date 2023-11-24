# Precompile benchmarks

This file documents a simple research on the performance of precompiled contracts.

## Methodology

In the current version of our contracts, we don't have any benchmarks on our precompiled contracts. This is a problem because it exposes us to the risk of DDoS, storage exhaustion and other attacks. In order to mitigate this risk, we need to have a better understanding of the performance of our precompiled contracts.

Frontier offers some manual way of recording cost of execution via `PrecompileHandle`.

## Results

| **method**          | **interface** | **weight**      |  **cost**       |
| ---------------- | ----------- | ----------- | ----------- |
| `create_collection` | extrinsic  | {ref_time: 360,414,000; proofSize: 1,493}  | 0.000942882 |    
| `create_collection` | precompile | {ref_time: 1,056,325,000; proofSize: 2154} | 0.000154469 |
| `mint`              | precompile | {ref_time: 613,225,000, proofSize: 790 }   | 0.000090751 |
| `mint`              | extrinsic  | {ref_time: 295,385,471; proofSize: 4,051}  | 0.001522267 |
