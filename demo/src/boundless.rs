use alloy::signers::local::PrivateKeySigner;
use anyhow::{Context, Result, anyhow};
use boundless_market::{
    Client,
    contracts::{FulfillmentData, Predicate},
    request_builder::{OfferParams, RequirementParams},
    storage::StorageProviderConfig,
};
use clap::Parser;
use guest::{ECHO_ELF, ECHO_ID};
use std::time::Duration;
use url::Url;

#[derive(Parser, Clone, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// URL of the Ethereum RPC endpoint.
    #[clap(short, long, env)]
    rpc_url: Url,
    /// Private key used to interact with the contract.
    #[clap(short, long, env)]
    private_key: PrivateKeySigner,
    /// Storage provider to use
    #[clap(flatten, next_help_heading = "Storage Provider")]
    storage_config: StorageProviderConfig,

    #[clap(flatten, next_help_heading = "Boundless Market Deployment")]
    boundless_deployment: Option<boundless_market::Deployment>,

    #[clap(long, default_value = "stark")]
    proof_type: String,
}
impl Args {
    /// Main logic which creates the Boundless client, executes the proofs and submits the tx.
    pub async fn run(self) -> Result<()> {
        tracing::info!("Starting Boundless Example");
        // Create a Boundless client from the provided parameters.
        let client = Client::builder()
            .with_rpc_url(self.rpc_url)
            .with_deployment(self.boundless_deployment)
            .with_storage_provider_config(&self.storage_config)?
            .with_private_key(self.private_key)
            .build()
            .await
            .context("failed to build boundless client")?;

        let echo_message = [
            1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32,
        ];

        let blake3_claim_digest =
            shrink_bitvm2::ShrinkBitvm2ReceiptClaim::ok(ECHO_ID, echo_message.to_vec())
                .claim_digest();

        tracing::info!("Blake3 claim digest: {:?}", blake3_claim_digest);

        // Build the request based on whether program URL is provided
        let request = client
            .new_request()
            .with_requirements(
                RequirementParams::builder()
                    .predicate(Predicate::claim_digest_match(blake3_claim_digest)),
            )
            .with_offer(
                OfferParams::builder()
                    .min_price(alloy::primitives::utils::parse_ether("0.0017")?)
                    .max_price(alloy::primitives::utils::parse_ether("0.002")?)
                    .lock_collateral(alloy::primitives::utils::parse_ether("0")?)
                    .timeout(1000)
                    .lock_timeout(1000),
            )
            .with_stdin(echo_message)
            .with_program(ECHO_ELF)
            .with_shrink_bitvm2_proof();

        let (request_id, expires_at) = client.submit_onchain(request).await?;

        // Wait for the request to be fulfilled. The market will return the journal and seal.
        tracing::info!("Waiting for request {:x} to be fulfilled", request_id);
        let fulfillment = client
            .wait_for_request_fulfillment(
                request_id,
                Duration::from_secs(5), // check every 5 seconds
                expires_at,
            )
            .await?;
        let fulfillment_data = fulfillment.data()?;
        tracing::info!("Fulfillment data: {:?}", fulfillment.data()?);
        tracing::info!("Request {:x} fulfilled", request_id);

        if !matches!(fulfillment_data, FulfillmentData::None) {
            return Err(anyhow!("Fulfillment data should be none"));
        }

        let seal = fulfillment.seal;
        tracing::info!("Seal length: {}", seal.len());
        let proof = shrink_bitvm2::Groth16Seal::decode(&seal[4..])?;
        tracing::info!("Verifying proof...");
        shrink_bitvm2::verify::verify(&proof, blake3_claim_digest)?;
        tracing::info!("Proof verified successfully");
        Ok(())
    }
}
