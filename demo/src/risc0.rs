use anyhow::Result;
use clap::Parser;
use guest::{ECHO_ELF, ECHO_ID};
use risc0_zkvm::{ExecutorEnv, ProverOpts, default_prover, sha::Digestible};

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
        let blake3_receipt = blake3_groth16::compress_blake3_groth16(&receipt)
            .await
            .unwrap();
        blake3_receipt.verify(ECHO_ID).expect("verification failed");

        let expected_claim_digest =
            blake3_groth16::Blake3Groth16ReceiptClaim::ok(ECHO_ID, input.to_vec()).digest();
        assert!(
            blake3_receipt.claim_digest()? == expected_claim_digest,
            "claim digest mismatch"
        );
        tracing::info!("done");
        Ok(())
    }
}
