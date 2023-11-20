# Precompile benchmarks

This file documents a simple research on the performance of precompiled contracts.

## Methodology

In the current version of our contracts, we don't have any benchmarks on our precompiled contracts. This is a problem because it exposes us to the risk of DDoS, storage exhaustion and other attacks. In order to mitigate this risk, we need to have a better understanding of the performance of our precompiled contracts.

Frontier offers some manual way of recording cost of execution via `PrecompileHandle`.

## Results

