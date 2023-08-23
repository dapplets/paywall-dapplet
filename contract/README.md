# NEAR Paywall Smart Contract

This repository contains a smart contract written in Rust for the NEAR Protocol that implements a basic paywall functionality for content purchase.

## Description

The smart contract allows users to purchase access to specific content using the NEAR Protocol. It keeps track of purchased content for each user and ensures that content is not purchased multiple times by the same user.

## Usage

Users can call the `buy` function to purchase access to content by providing the `content_id` as a parameter. The function is marked as `payable`, which means users can attach NEAR tokens to the transaction to complete the purchase. If the purchase is successful, the attached tokens are returned to the user.

The `purchased` function can be used to check whether a specific account has purchased access to a particular content.

The `purchases` function allows users to retrieve a list of content they have purchased.

## Quickstart

1. Make sure you have installed [rust](https://rust.org/).
2. Install the [`NEAR CLI`](https://github.com/near/near-cli#setup)

### 1. Build the smart contract

```bash
./build.sh
```

### 2. Run tests

```bash
./test.sh
```

### 3. Deploy the contract

```bash
NEAR_ENV=mainnet ./deploy.sh
```

