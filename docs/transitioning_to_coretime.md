# Transitioning LAOS from Parachain Lease to Agile Coretime

## Overview

LAOS (ParaId **3370**, Core **8** on Polkadot) is currently operating under a **legacy PLO slot lease** that expires at:

| Parameter | Value |
|---|---|
| Relay chain block | **~31,472,400** |
| Coretime chain timeslice | **~404,640** |
| Estimated calendar date | **~May 8, 2026** |
| Time remaining (from document creation) | ~18 days |

When this lease expires, LAOS will **stop producing blocks** unless bulk coretime has been purchased and assigned beforehand. This document explains what actions are required and who can take them.

---

## Background: Agile Coretime

Polkadot replaced the legacy parachain slot auction model with **Agile Coretime** in September 2024.
Under this new model:

- Parachains no longer need to win multi-year auctions — they **purchase coretime** in 28-day periods ("bulk coretime") on the Polkadot Coretime Chain.
- Existing parachains that held slot leases (like LAOS) were automatically migrated: their lease was kept valid until its natural expiry, but they now need to buy coretime to continue after that date.
- Because LAOS won auction slot, it holds a **priority renewal right** — it can renew its core assignment before the open sales period begins, at a predictable price, without competing.

There are two types of coretime:

- **Bulk coretime**: 28-day dedicated core assignment. This is the like-for-like replacement for a legacy slot lease. Purchased once per period.
- **On-demand coretime**: pay-per-block model, useful for low-frequency chains. Not recommended for LAOS at this stage.

---

## Cost of Coretime (Live On-Chain Data, April 20, 2026)

The following was queried live from the Polkadot Coretime Chain (`broker` pallet, chain block #4,078,907):

### LAOS Renewal Price (from `broker.potentialRenewals`)

| Field | Value |
|---|---|
| ParaId | 3370 (LAOS) |
| Core | **8** |
| Renewal timeslice | 393,405 |
| **Renewal price** | **103,000,000,000 Planck = 10.3 DOT** |

This is the **exact price** LAOS will pay if it exercises its priority renewal right. It is locked in and independent of market demand — LAOS does not compete with other buyers.

### Current Market Context (from `broker.saleInfo`)

| Field | Value |
|---|---|
| Sale start (relay block) | 30,766,791 |
| Floor price (`endPrice`) | 100,000,000,000 Planck = **10 DOT** |
| Last sellout price | ~115,927,407,430 Planck = **~11.59 DOT** |
| Cores offered this period | 77 |
| Cores sold so far | 22 of 77 |

The market is currently under-subscribed (22/77 cores sold), which has pushed the floor to **10 DOT**. LAOS's renewal price of 10.3 DOT is slightly above the current floor, which is expected for a renewing chain (the 3% annual bump described below).

### Price Trajectory (from `broker.configuration`)

| Parameter | Value |
|---|---|
| Region length | 5,040 timeslices = **28 days** |
| Interlude length | 100,800 relay blocks = **~7 days** |
| Lead-in length | 201,600 relay blocks = **~14 days** |
| **Renewal bump** | **3% per period** |

Each time LAOS renews, its price increases by **3%**. If the market floor stays at 10 DOT, the price trajectory is:

| Period | Approximate renewal price |
|---|---|
| Next (May 2026) | **10.3 DOT** |
| June 2026 | ~10.61 DOT |
| July 2026 | ~10.93 DOT |
| August 2026 | ~11.26 DOT |
| 1 year out (April 2027) | ~13.4 DOT |

> **Note**: If the open market price rises above the renewal price, LAOS would pay the market rate instead. If the floor drops below LAOS's renewal price, LAOS still pays the 3%-bumped renewal price. Prices are capped by the market floor from below and uncapped above.

### USD Estimate

At DOT = $5 (approximate as of document creation, verify before transacting):

| Period | DOT | USD equivalent |
|---|---|---|
| Next renewal | 10.3 DOT | ~$52 |
| Annual ongoing cost | ~130 DOT/year | ~$650/year |

This is an extremely low operational cost. The main risk is not the price but **missing the renewal window**.

---

## Quick Purchase Guide (For Dummies)

This is the simplest path to renewing LAOS coretime. You only need a browser and **any Polkadot account with enough DOT** — no special permissions are required. `broker.renew()` is permissionless: anyone can pay for the renewal and it takes effect for LAOS regardless of who submits it.

### What you'll need before starting

- [ ] **A Polkadot wallet browser extension** installed — recommended: [Talisman](https://talisman.xyz) or [SubWallet](https://subwallet.app). Both are free and work on Chrome/Brave/Firefox.
- [ ] **Your own account** already set up in the extension (any Polkadot-compatible account will do — it does not need to be affiliated with LAOS or the LAOS Foundation).
- [ ] **At least 11 DOT available** in your account on the **Polkadot Relay Chain** (10.3 DOT for the renewal + ~0.5 DOT for transaction fees). If your DOT is already on the Coretime Chain, skip Step 2.

> [!NOTE]
> The prices shown on Lastic's website are displayed in a different format than the raw on-chain values. Always confirm the exact amount in your wallet's signing popup before approving.

---

### Step 1 — Install a wallet and set up your account

1. Go to [talisman.xyz](https://talisman.xyz) and install the browser extension.
2. Open Talisman and set up your account:
   - **New account**: click **"Add Account"** → **"Create a new account"** and follow the prompts.
   - **Existing account**: click **"Add Account"** → **"Import via Recovery Phrase"** (or Import JSON) and enter your seed phrase.
3. Make sure your account shows a DOT balance on the Polkadot network. If it shows zero, ensure you have DOT on-chain and are looking at the right network.

---

### Step 2 — Move DOT to the Coretime Chain (Teleport)

Coretime is paid on the **Polkadot Coretime Chain**, not the Relay Chain. You need to teleport DOT across first.

1. Go to **[https://www.lastic.xyz/polkadot/teleport](https://www.lastic.xyz/polkadot/teleport)**
2. Click **"CONNECT WALLET"** in the top-right corner and approve the connection in Talisman.
3. Make sure **"Network: Polkadot Coretime"** is shown in the top-right dropdown. If not, click it and switch.
4. On the Teleport page you'll see:
   - **From**: Polkadot (Relay Chain)
   - **To**: Polkadot Coretime Chain
5. Enter the amount: **`11`** DOT (this covers the 10.3 DOT cost + fees)
6. Click **"TELEPORT"** and approve the transaction in Talisman.
7. Wait ~30–60 seconds. The DOT will arrive on the Coretime Chain.

> If you already have DOT on the Coretime Chain (check via [Talisman's portfolio](https://app.talisman.xyz) — look for "Polkadot Coretime" network), skip this step.

---

### Step 3 — Renew LAOS coretime on Lastic

1. Go to **[https://www.lastic.xyz/polkadot/renewal](https://www.lastic.xyz/polkadot/renewal)**
2. Make sure your wallet is still connected (look for the address in the top-right; if it says "CONNECT WALLET", click it again).
3. You'll see:
   - A **timeline bar** showing the current sale phase ("Renewal Period" / "Linearly decreasing price" / "Stable price")
   - A table titled **"CORES THAT NEED TO BE RENEWED!"** listing chains by Para ID
4. Scroll down through the table pages (use the **"Next"** button at the bottom) to find **Para ID 3370**.
5. Verify the row shows:
   - **Para ID**: 3370
   - **Core**: 8
   - **Price**: ~10.3 DOT
6. Click the **"RENEW"** button on that row.
7. Talisman will pop up asking you to sign the transaction. Review the details and click **"Approve"**.
8. Done. The renewal is complete once the transaction is included (usually within 30 seconds).

> [!IMPORTANT]
> You must renew while the sale is **not yet sold out**. The Lastic timeline shows where in the current period you are. The "Renewal Period" at the start of each cycle is the safest window. Do NOT wait until the lease expiry date.

---

### Step 4 — Verify the renewal worked

1. Go to **[https://www.lastic.xyz/polkadot/paraId-execution](https://www.lastic.xyz/polkadot/paraId-execution)**
2. In the **"ParaId"** search box, type **`3370`** and press Enter.
3. The row for 3370 should show status **"Currently Active"** with an updated lease/region period.
4. You can also check on Subscan: **[https://polkadot.subscan.io/parachain/3370](https://polkadot.subscan.io/parachain/3370)** — scroll to "Associated Coretime" to see the new assignment.

---

### Timeline: when can you renew?

The Lastic timeline on the renewal page shows four phases per 28-day cycle:

| Phase | When | What happens |
|---|---|---|
| **Renewal Period** | Days 1–7 of each cycle | ✅ Best time — fixed priority price (10.3 DOT for LAOS) |
| **Linearly Decreasing Price** | Days 8–21 | ✅ Can still renew, but prices fluctuate with Dutch auction |
| **Stable Price** | Days 21–28 | ✅ Can still renew at floor price (10 DOT) |
| **Sold Out** | If all 77 cores are taken | ❌ Cannot renew until next cycle — avoid this! |

The current sale has 55 cores unsold (22/77 sold), so there is plenty of room — but renewals still need to happen within the cycle to take effect before the legacy lease expires on ~May 8.

---

## What the Node Code Needs (Answer: Nothing, for now)

The current LAOS collator node uses `cumulus_client_consensus_aura::collators::basic` — a slot-based Aura collator. This is **fully compatible with bulk coretime**, which presents the same continuous core assignment that the node currently sees under the legacy lease.

| Component | Current state | Action needed |
|---|---|---|
| `service.rs` collator (`collators::basic`) | Compatible with bulk coretime | ✅ None for this transition |
| `ConsensusHook` (`FixedVelocityConsensusHook`) | Compatible | ✅ None |
| `CheckAssociatedRelayNumber` (`RelayNumberStrictlyIncreases`) | Compatible | ✅ None |
| SDK branch (`stable2409-laos`) | Supports coretime primitives | ✅ None |
| Runtime pallets | No coretime-specific pallets required | ✅ None |

> **Note**: If LAOS ever moves to **on-demand coretime** or **elastic scaling** (multiple simultaneous cores), the node will need to be upgraded to `collators::lookahead`. That is not required for this transition.

**The only required action is the on-chain purchase of bulk coretime**, described below.

---

## Who Can Purchase Coretime

The `broker.renew()` extrinsic on the Polkadot Coretime Chain is a **fully permissionless call** — any account with sufficient DOT can submit it for any ParaId, including LAOS (ParaId 3370). There is no whitelist, no manager check, and no on-chain restriction on who the signer is.

### Any Individual With Enough DOT

If you hold at least ~11 DOT on the Polkadot Coretime Chain (or on the Relay Chain, which can be teleported across), you can submit `broker.renew(8)` from your own account. The DOT will be deducted from your account, and the new 28-day bulk coretime assignment for LAOS will take effect at the next timeslice — regardless of your identity.

This means:
- Community members, supporters, or anyone who wants LAOS to keep running can independently fund and submit the renewal.
- No access to any LAOS-controlled account is needed.
- No coordination with the LAOS Foundation is required beforehand (though notifying the team is appreciated).

### Governance Path (for completeness)

Because `pallet_sudo` has been removed from LAOS, on-chain governance controls root-level actions. However, `broker.renew()` does **not** require root — it requires only a signed origin with enough DOT. The governance path is therefore not needed for coretime renewal and is only mentioned here for completeness.

---

## How to Purchase Bulk Coretime (Step by Step)

### Prerequisites

- **Any Polkadot account** loaded in a browser wallet extension (e.g. [Talisman](https://talisman.xyz) or [SubWallet](https://subwallet.app)) — no special permissions required
- **At least ~11 DOT** available in that account on the **Polkadot Coretime Chain** (10.3 DOT for the renewal + ~0.5 DOT for fees). If your DOT is on the Relay Chain, teleport it first (see Quick Purchase Guide → Step 2)
- A browser with [Polkadot.js Apps](https://polkadot.js.org/apps) open and connected to the Coretime Chain

### Step 1 — Connect to the Polkadot Coretime Chain

Navigate to:

```
https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fsys.ibp.network%2Fcoretime-polkadot#/extrinsics
```

Make sure the network selector shows **"Polkadot Coretime"**, not the Relay Chain or LAOS.

Alternatively, connect manually:
1. Open Polkadot.js Apps → click the network logo (top left)
2. Search for "Coretime" and select **Polkadot Coretime Chain**

### Step 2 — Check the Current Sale / Interlude Phase

Before submitting, verify the sale is in the **Interlude or Renewal** period (this is when priority renewals can be submitted, before the open market sale begins):

1. Go to **Developer → Chain State**
2. Select pallet `broker`, query `saleInfo()`
3. Check `saleStart` vs. current block, and whether a `leadinLength` or `interludeLength` period is active

The renewal window typically opens a few days before the sale period ends. **Do not wait until the lease expires** — submit the renewal as early as possible.

### Step 3 — Verify LAOS Has a Renewal Right

1. Under **Developer → Chain State**, select pallet `broker`
2. Query `potentialRenewals()` — look for an entry with `task: 3370` (LAOS ParaId)
3. This entry shows the `price` and the `core` that can be renewed

If visible, LAOS holds a priority renewal right for **Core 8**.

### Step 4 — Submit the `broker.renew()` Extrinsic

1. Go to **Developer → Extrinsics**
2. In the **signing account** dropdown, select **your own account** (the one with DOT on the Coretime Chain)
3. Select pallet: **`broker`**
4. Select extrinsic: **`renew`**
5. Fill in the parameter:
   - `core`: **`8`**
6. Click **Submit Transaction** and sign with your account's key

The transaction will:
- Deduct the coretime price (10.3 DOT) from **your** signing account
- Register a new 28-day bulk coretime assignment for ParaId 3370 on Core 8
- Take effect at the next timeslice boundary, ensuring seamless continuity for LAOS block production

### Step 5 — Verify Assignment

After the extrinsic is included:

1. Check **Developer → Chain State → broker → workload(8)** — should show `task: 3370` for the upcoming timeslice
2. Check on Subscan: https://polkadot.subscan.io/parachain/3370 — "Associated Coretime" section should update within the next few blocks

You can also use [Lastic.xyz](https://www.lastic.xyz) to monitor the coretime assignment visually.

---

## Timeline and Urgency

| Calendar date | Action |
|---|---|
| Now (April 20, 2026) | Confirm renewal right exists via `broker.potentialRenewals()` |
| **ASAP** | Submit `broker.renew(8)` from any account with ≥11 DOT on the Coretime Chain |
| ~May 8, 2026 | Relay block 31,472,400 — lease expires. If renewal was submitted: block production continues uninterrupted. If not: LAOS stops producing blocks. |

> [!CAUTION]
> Do not wait until the lease expiry block to act. Coretime sales operate in time windows and the renewal right may need to be exercised during a specific interlude period before the block is hit.

---

## Summary

| Question | Answer |
|---|---|
| Does the node code need a software update? | **No** — current code is compatible with bulk coretime |
| What action is required? | Submit `broker.renew(8)` on the Polkadot Coretime Chain |
| Who can submit it? | **Anyone** — `broker.renew()` is permissionless; any account with enough DOT on the Coretime Chain can do it |
| What chain is the extrinsic submitted on? | **Polkadot Coretime Chain** (not the LAOS parachain, not the Relay Chain) |
| When does the lease expire? | ~May 8, 2026 (relay block ~31,472,400, timeslice ~404,640) |
| **Cost of next renewal** | **10.3 DOT** (~$52 at $5/DOT) — locked in via priority renewal right |
| Ongoing annual cost | ~130 DOT/year at 3% bump per period (~$650/year at $5/DOT) |
| Will nodes need a restart? | No restart is required — the transition is transparent to collators |
| Future work recommended? | Upgrade to `collators::lookahead` for on-demand/elastic coretime support |
