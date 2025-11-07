use anyhow::Result;
use clap::Parser;
use guest::{ECHO_ELF, ECHO_ID};
use risc0_zkvm::{ExecutorEnv, Groth16Seal, ProverOpts, default_prover, sha::Digestible};

#[derive(Parser, Clone, Debug)]
pub struct Args {}

impl Args {
    pub async fn run(self) -> Result<()> {
        let input = [3u8; 32];

        let receipt = tokio::task::spawn_blocking(move || {
            let env = ExecutorEnv::builder().write_slice(&input).build().unwrap();
            tracing::info!("Proving echo program to get succinct receipt");
            // Produce a receipt by proving the specified ELF binary.
            default_prover()
                .prove_with_opts(env, ECHO_ELF, &ProverOpts::succinct())
                .unwrap()
                .receipt
        })
        .await
        .unwrap();
        tracing::info!("Initial receipt created, compressing to blake3_groth16");
        let groth16_receipt = blake3_groth16::compress_blake3_groth16(&receipt)
            .await
            .unwrap();
        let groth16_receipt = groth16_receipt.inner.groth16().unwrap();
        let blake3_g16_seal = Groth16Seal::decode(&groth16_receipt.seal).unwrap();
        let blake3_claim_digest =
            blake3_groth16::Blake3Groth16ReceiptClaim::ok(ECHO_ID, input.to_vec()).digest();
        blake3_groth16::verify::verify(&blake3_g16_seal, blake3_claim_digest)
            .expect("verification failed");
        tracing::info!("done");
        Ok(())
    }
}
