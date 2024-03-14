#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
near deploy --wasmFile ./target/wasm32-unknown-unknown/release/paywall_dapplet.wasm --masterAccount v2.paywall.near --accountId v2.paywall.near
