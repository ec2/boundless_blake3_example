# Boundless Blake3 Groth16 Proof Example

Currently, Boundless supports the standard Risc0 Groth16 compression which outputs the ReceiptClaim which can be hashed using SHA2 to get the claim digest. This is not ideal for environments which SHA2 hashing may be costly or impossible. The Chainway Labs team has written a new Groth16 circuit which has a single public output which is the BLAKE3 claim digest of a Risc0 execution.

This repo contains an example of using the Boundless SDK to request a proof with this new proof type.

## Setup

Clone the Boundless repo and checkout the following branch:

```
ec2/shrink_bitvm2
```

We need to download the SRS files, circom, etc... to generate setup files required by the prover. To do that, run:

```
cargo xtask-setup-blake3-groth16 <BLAKE3_GROTH16_SETUP_DIR>
```

`BLAKE3_GROTH16_SETUP_DIR` is where all the artifacts needed for proving will go. It is also the name of the environment variable that the prover will use, so make sure to export it.

## Proving

There are 3 examples in this repo.

### 1. Local proving

This will locally prove the SNARK and Blake3 Groth16 proofs. You can run that by running:

```
export BLAKE3_GROTH16_SETUP_DIR="<BLAKE3_GROTH16_SETUP_DIR>"
just prove-local
```

### 2. Bento proving

This runs the same example as above except it will use Bento as the prover. To start Bento, run this from the Boundless repository. Make sure to specify the Docker files, or it will use the prebuild binaries.

```
export BLAKE3_GROTH16_SETUP_DIR="<BLAKE3_GROTH16_SETUP_DIR>"
REST_API_DOCKERFILE=dockerfiles/rest_api.dockerfile AGENT_DOCKERFILE=dockerfiles/agent.dockerfile just bento up
```

Then you can request a proof from bento by running:

```
just prove-bento
```

### 3. Boundless Market Proving

The following instructions are for setting up a local deployment of the Boundless marketplace.

Then, we need to deploy the market and launch the Broker and Bento.

```
# This is the directory in which groth16 setup artifacts live.
export BLAKE3_GROTH16_SETUP_DIR=<BLAKE3_GROTH16_SETUP_DIR>

# Starts Anvil and deploy the market and verifier contracts
RISC0_DEV_MODE=0 just localnet

# Exports all environment of the boundless local network.
source .env.localnet

# For testing, we locally build the docker images for the Bento and the Boundless broker
BOUNDLESS_MINING=false REST_API_DOCKERFILE=dockerfiles/rest_api.dockerfile BROKER_DOCKERFILE=dockerfiles/broker.dockerfile AGENT_DOCKERFILE=dockerfiles/agent.dockerfile just prover up .env.localnet
```

At this point, a Boundless market should be deployed locally and Bento should be started as well. To request an order through the market, run:

```
# Exports all environment of the boundless local network.
source .env.localnet

just prove-boundless
```
