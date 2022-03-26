Counter Contract Tutorial
===================

This repository include a example implementaion of Rust contract with NEAR-SDK, a contract which use basic contract, versioned and upgradeable contract

Prerequisites
=============
If you're using Gitpod, you can skip this step.

  * Make sure Rust is installed per the prerequisites in [`near-sdk-rs`](https://github.com/near/near-sdk-rs).
  * Make sure [near-cli](https://github.com/near/near-cli) is installed.

Building this contract
======================
Run the following, and we'll build our rust project up via cargo. This will generate our WASM binaries into our `out/` directory. This is the smart contract we'll be deploying onto the NEAR blockchain later.
```bash
./build.sh
```

Testing this contract
=====================
We have some tests that you can run. For example, the following will run our simple tests to verify that our contract code is working.
```bash
cargo test -- --nocapture
```
The more complex simulation tests aren't run with this command, but we can find them in `tests/sim`.

Using this contract
===================

### Quickest deploy

You can build and deploy this smart contract to a development account. [Dev Accounts](https://docs.near.org/docs/concepts/account#dev-accounts) are auto-generated accounts to assist in developing and testing smart contracts. Please see the [Standard deploy](#standard-deploy) section for creating a more personalized account to deploy to.

```bash
near dev-deploy --wasmFile out/counter-tutorial.wasm
```

Behind the scenes, this is creating an account and deploying a contract to it. On the console, notice a message like:

>Done deploying to dev-1234567890123

In this instance, the account is `dev-1234567890123`. A file has been created containing a key pair to
the account, located at `neardev/dev-account`. To make the next few steps easier, we're going to set an
environment variable containing this development account id and use that when copy/pasting commands.
Run this command to set the environment variable:

```bash
source neardev/dev-account.env
```

You can tell if the environment variable is set correctly if your command line prints the account name after this command:
```bash
echo $CONTRACT_NAME
```

The next command will initialize the contract using the `new` method:

```bash
near call $CONTRACT_NAME new --accountId $ACCOUNT_NAME
```

To view the number in contract:

```bash
near view $CONTRACT_NAME get_num
```

To increment number

```bash
near call $CONTRACT_NAME increment --accountId $ACCOUNT_NAME
```
