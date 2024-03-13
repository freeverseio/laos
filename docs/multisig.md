# How to use multisig accounts

This guide explains how to use multisig accounts in Caladan.

## What is a multisig account?

A multisig account is an account that requires multiple signatures to approve a transaction. This is useful for security reasons, as it means that no single person can approve a transaction on their own. It is also useful for organizational reasons, as it means that multiple people can be involved in the approval process.

More about multisig accounts in Polkadot can be found [here](https://wiki.polkadot.network/docs/learn-account-multisig#introduction-to-multisig-accounts).
And more detailed version of this guide can be found [here](https://support.polkadot.network/support/solutions/articles/65000181826-how-to-create-and-use-a-multisig-account)

## Pre-requisites

We will be using a local Zombienet network to demonstrate:

1. Create a multisig account
2. Change sudo key to a multisig account
3. Approve a transaction with a multisig account

## Launch a local Zombienet network

Follow the instructions [here](../zombienet/README.md) to launch a local Zombienet network.

## Create a multisig account

In `Accounts` tab, click on `+ Multisig`:

![+Multisig](https://github.com/freeverseio/laos/assets/137785454/0e7b3bea-255d-4940-9cdc-28b948406fca)

In the new window, select signatories for the multisig and give a name to the multisig:

![Select signatories](https://github.com/freeverseio/laos/assets/137785454/579f41df-7c40-4119-950d-8028b3198966)

You can see the new account in the `multisig` section (in our example, it's called `Sudoers`):

![Sudoers](https://github.com/freeverseio/laos/assets/137785454/5ab13574-edcc-402e-8bdd-2a6b49c92a5c)

And more info when you click on it:

![More info](https://github.com/freeverseio/laos/assets/137785454/d1f136ed-b5cf-4760-876a-dba578f01586)

**NOTE** Next steps will require you to sign transactions and send funds from the multisig account, so make sure to deposit some funds to it.

## Change sudo to new multisig

Go to `Extrinsics` tab and select `Sudo->setKey()` extrinsic, and select newly created multisig account as `new`:

![Change sudo](https://github.com/freeverseio/laos/assets/137785454/6c71335b-4841-452c-972e-328686b40790)

**NOTE**: This should not be wrapped to `sudo` extrinsic and should be signed by the current sudo account (f.e, it's `Alith` in this example)

## Use multisig to transfer funds

Now we want use new multisig account to transfer funds. For this, simply click on the `Transfer` tab and select multisig account as a `sender`. Select `to` and `amount` and click on `Make Transfer`:

![Transfer](https://github.com/freeverseio/laos/assets/137785454/32b0fc93-2fc5-43f5-8bbc-503bb35a442a)

In the next window, you will see this:

![Authorize tx](https://github.com/freeverseio/laos/assets/137785454/df0593fd-6663-411a-821b-e7e0c4becb6c)

**NOTE**: Make sure to copy `multisig call data` field and share it with other multisig signatories.

It's important to note that this call will signed and submitted first with one of the signatories of the multisig, meaning it already has 1 approval. Then the `multisig call data` is shared with other signers. Now, click on `Sign and Submit`

## Approve multisig call

If you are one of the signers, when you go to `Accounts` tab and `multisig` section, you will see a notification that says `Multisig approvals pending`:

![Approvals pending](https://github.com/freeverseio/laos/assets/137785454/93c8750d-b714-4e3f-8638-818a74eb365c)

Click on it and you will go to a window like this:

![Pending call hashes](https://github.com/freeverseio/laos/assets/137785454/25b939e5-7327-4006-aaf1-3c583049ac81)

Here you have to now paste `multisig call data` from the previous step, select the signer and `Approve` the call.

## Multisig executed

Once the second signer approved the call, multisig will be executed and funds are transferred:

![Multisig Executed](https://github.com/freeverseio/laos/assets/137785454/6ce884e9-17ee-4a9d-b1d2-6c06b04e71ca)
