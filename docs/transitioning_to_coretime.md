# Transitioning LAOS from Parachain Lease to Agile Coretime

## Overview

LAOS (ParaId **3370**, Core **8** on Polkadot) is currently operating under a **legacy PLO slot lease** that expires at:

| Parameter | Value |
|---|---|
| Relay chain block | **~31,472,400** |
| Coretime chain timeslice | **393,405** |
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
- [ ] **At least 11 DOT available** in your account on the **Polkadot Coretime Chain** (10.3 DOT for the renewal + ~0.5 DOT for transaction fees). DOT on the Relay Chain or Asset Hub needs to be teleported first — see Step 2.

> [!IMPORTANT]
> **The renewal window does not open until ~May 8, 2026.** Attempting to submit `broker.renew(8)` before that date will fail with `Broker: NotAllowed`. See [When Exactly Can You Renew?](#when-exactly-can-you-renew) for a full explanation.

---

### Step 1 — Install a wallet and set up your account

1. Go to [talisman.xyz](https://talisman.xyz) and install the browser extension.
2. Open Talisman and set up your account:
   - **New account**: click **"Add Account"** → **"Create a new account"** and follow the prompts.
   - **Existing account**: click **"Add Account"** → **"Import via Recovery Phrase"** (or Import JSON) and enter your seed phrase.
3. Make sure your account shows a DOT balance on the Polkadot network. If it shows zero, ensure you have DOT on-chain and are looking at the right network.

---

### Step 2 — Move DOT to the Coretime Chain (Teleport)

Coretime is paid on the **Polkadot Coretime Chain**. If your DOT is already there, skip this step.

> Check via [Talisman's portfolio](https://app.talisman.xyz) — look for **"Polkadot Coretime"** in the network list. If it shows a balance, you're ready.

#### My DOT is on Asset Hub (direct teleport — recommended)

You can teleport directly from Asset Hub to the Coretime Chain in a single step using Polkadot.js Apps — no intermediate hop via the Relay Chain needed.

1. Go to [Polkadot.js Apps connected to Asset Hub](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpolkadot-asset-hub-rpc.polkadot.io#/accounts)
2. Navigate to **Accounts → Teleport**
3. Select your account as the sender
4. Set **destination chain** to **Polkadot Coretime Chain**
5. Enter amount: **`11`** DOT
6. Click **Teleport** and sign with your wallet
7. Wait ~30–60 seconds. The DOT will arrive on the Coretime Chain.

> [!NOTE]
> If Polkadot.js Apps does not offer "Polkadot Coretime Chain" as a teleport destination from Asset Hub, use the two-hop path: Asset Hub → Relay Chain first (via **Accounts → Teleport**, destination: Polkadot Relay Chain), then Relay Chain → Coretime Chain (see below).

#### My DOT is on the Relay Chain

1. Go to [Polkadot.js Apps connected to the Relay Chain](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.ibp.network%2Fpolkadot#/accounts)
2. Navigate to **Accounts → Teleport**
3. Set **destination chain** to **Polkadot Coretime Chain**
4. Enter amount: **`11`** DOT and sign
5. Wait ~30–60 seconds.

Alternatively, use Lastic's teleport UI: **[https://www.lastic.xyz/polkadot/teleport](https://www.lastic.xyz/polkadot/teleport)** (note: Lastic only reads Relay Chain balances, not Asset Hub).

---

### Step 3 — Renew LAOS coretime via Polkadot.js Apps

> [!IMPORTANT]
> **The renewal window opens ~May 8, 2026** — not before. Attempting to submit earlier will fail with `Broker: NotAllowed`. See [When Exactly Can You Renew?](#when-exactly-can-you-renew) for a full explanation of why.

Once the window is open (confirm via the check in Step 2 of the advanced guide below), submit the renewal:

1. Go to:
   ```
   https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fsys.ibp.network%2Fcoretime-polkadot#/extrinsics
   ```
2. In the **signing account** dropdown, select your account (the one with DOT on the Coretime Chain).
3. Select pallet: **`broker`**
4. Select extrinsic: **`renew`**
5. Fill in the parameter — `core`: **`8`**

   > **Why core 8?** Coretime is assigned to physical compute slots called *cores*, not directly to parachains. Core 8 is the slot currently assigned to LAOS (ParaId 3370). `broker.renew(8)` tells the chain to renew that slot's assignment — the chain already knows Core 8 belongs to LAOS.

6. Click **Submit Transaction** and sign with Talisman.
7. Wait ~30 seconds for inclusion.

> [!NOTE]
> **Why not Lastic?** Lastic's renewal table only shows chains whose renewal period matches the *current* sale's `regionBegin`. LAOS's renewal is for the *next* period (`when: 393,405`), so it will not appear in Lastic's table until the new sale starts on ~May 8. Polkadot.js Apps is always reliable.

---

### Step 4 — Verify the renewal worked

1. In Polkadot.js Apps (still on Coretime Chain), go to **Developer → Chain State → broker → workload(8)**.
   - The result should include `task: 3370` for the upcoming timeslice.
2. Check on Subscan: **[https://polkadot.subscan.io/parachain/3370](https://polkadot.subscan.io/parachain/3370)** — scroll to "Associated Coretime" to see the new assignment.
3. You can also check [Lastic ParaID Execution](https://www.lastic.xyz/polkadot/paraId-execution) — search for `3370` and confirm status **"Currently Active"**.

---

### When Exactly Can You Renew?

The `broker.renew()` extrinsic uses the **broker pallet's timing model** and has two hard requirements:

1. The current sale must be in its **Interlude phase** (the first ~7 days of each sale cycle, before the open market begins)
2. The current sale's `regionBegin` must match LAOS's `potentialRenewals.when` (**393,405**)

As of April 22, 2026, the active sale has:

```
broker.saleInfo:
  saleStart:    30,766,791   ← open market already started (interlude is over)
  regionBegin:  388,365      ← does NOT match LAOS's when: 393,405
  regionEnd:    393,405
```

Both conditions fail today: the interlude for this sale has already ended, and the `regionBegin` (388,365) does not match LAOS's renewal key (393,405). This is why `broker.renew(8)` returns `Broker: NotAllowed`.

**The window opens when the next sale starts**, which happens at relay block **393,405 × 80 = 31,472,400 ≈ May 8, 2026** — the same moment the current region ends. The next sale's interlude then runs for ~7 days:

| Event | Relay block | Approx. date |
|---|---|---|
| Current region ends → **renewal window opens** | **31,472,400** | **~May 8, 2026** |
| Interlude ends → open market begins | ~31,573,200 | ~May 15, 2026 |
| Next region ends (next renewal due) | ~31,875,600 | ~June 12, 2026 |

**How to confirm the window is open** before submitting:

1. Go to **Developer → Chain State → broker → saleInfo()** on the Coretime Chain
2. Check `regionBegin` — it must equal **`393,405`** (not 388,365)
3. Check the current relay block — it must be **less than** `saleStart` (meaning the interlude is still active)

If both conditions are true, `broker.renew(8)` will succeed.

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

### Step 2 — Confirm the Renewal Window Is Open

Before submitting, verify that the next sale has started and its interlude is active. **This is the most important check — submitting outside this window always fails with `Broker: NotAllowed`.**

1. Go to **Developer → Chain State**
2. Select pallet `broker`, query `saleInfo()`
3. Verify:
   - `regionBegin` = **`393,405`** (if it still shows 388,365, the window has not opened yet)
   - Current relay block < `saleStart` (if current block ≥ saleStart, the interlude has ended and only the open market is active)

The window opens at relay block **~31,472,400 (~May 8, 2026)**. Before that date, `broker.renew(8)` will always be rejected.

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

| Calendar date | Relay block | Action |
|---|---|---|
| Now (April 22, 2026) | ~31,242,000 | Confirm `broker.potentialRenewals()` shows `core: 8, task: 3370`. Get DOT onto the Coretime Chain now (teleport from Asset Hub or Relay Chain). |
| Now → May 8 | 31,242,000 → 31,472,400 | **Cannot renew yet** — `broker.renew(8)` returns `NotAllowed`. Current sale's `regionBegin` is 388,365, not 393,405. |
| **~May 8, 2026** | **31,472,400** | **Renewal window opens.** New sale starts with `regionBegin: 393,405`. Submit `broker.renew(8)` immediately. |
| May 8 → May 15 | 31,472,400 → 31,573,200 | Interlude period — **safest renewal window** at fixed price (10.3 DOT). |
| May 15 → ~June 5 | 31,573,200 → ~31,875,600 | Open market phase — can still renew but price may vary. |
| **May 8 + no renewal** | 31,472,400+ | ❌ LAOS stops producing blocks if `broker.renew(8)` was never submitted. |

> [!CAUTION]
> The renewal window opens **the same day the lease expires** (~May 8). Someone must be ready to submit `broker.renew(8)` on that date. Missing the 7-day interlude window (May 8–15) is the highest risk — after that, open market prices apply and cores could sell out.

> [!TIP]
> Prepare everything before May 8: install Talisman, get DOT onto the Coretime Chain, and have the Polkadot.js Apps extrinsic page bookmarked. On May 8, just check that `saleInfo().regionBegin = 393,405` and submit.

---

## Summary

| Question | Answer |
|---|---|
| Does the node code need a software update? | **No** — current code is compatible with bulk coretime |
| What action is required? | Submit `broker.renew(8)` on the Polkadot Coretime Chain |
| Who can submit it? | **Anyone** — `broker.renew()` is permissionless; any account with enough DOT on the Coretime Chain can do it |
| What chain is the extrinsic submitted on? | **Polkadot Coretime Chain** (not the LAOS parachain, not the Relay Chain) |
| When does the lease expire? | ~May 8, 2026 (relay block ~31,472,400, timeslice 393,405) |
| **When does the renewal window open?** | **~May 8, 2026** — when `broker.saleInfo().regionBegin` changes to `393,405` |
| Why does `broker.renew()` fail before May 8? | The broker pallet looks up `potentialRenewals[core=8, when=regionBegin]`. Today's sale has `regionBegin=388,365`, not `393,405` — so the lookup fails with `NotAllowed` |
| **Cost of next renewal** | **10.3 DOT** (~$52 at $5/DOT) — locked in via priority renewal right |
| Ongoing annual cost | ~130 DOT/year at 3% bump per period (~$650/year at $5/DOT) |
| Will nodes need a restart? | No restart is required — the transition is transparent to collators |
| Future work recommended? | Upgrade to `collators::lookahead` for on-demand/elastic coretime support |
