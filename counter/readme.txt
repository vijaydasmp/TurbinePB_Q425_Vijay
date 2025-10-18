A simple Solana on-chain counter using Rust and Node.js.
This project has three main parts:

lib.rs – My on-chain program (smart contract) that increments a counter.

initCounter.js – A Node.js script I use to initialize a counter account on my local Solana cluster.

client.js – A Node.js client that increments the counter.


Steps to build and deploy

Build the Rust program:
cargo build-bpf --manifest-path=<toml path> --bpf-out-dir=<output target directory>

Deploy it to localnet:
solana program deploy <so path>

Copy the Program ID and update it in initCounter.js and client.js:
const programId = new PublicKey("<MY_PROGRAM_ID>")

Then Initializing the Counter Account:
I run initCounter.js to create a new counter account:
node client/initCounter.js

Incrementing the Counter:
I run client.js to increment the counter:
node client/client.js

In Solana logs its visibe the incremented counter value
