BENTO_API_URL := env_var_or_default("BENTO_API_URL",  "http://localhost:8081")
BONSAI_API_URL := env_var_or_default("DEFAULT_BENTO_API_URL", BENTO_API_URL)

S3_URL := env_var_or_default("S3_URL", "http://localhost:9000")
S3_BUCKET := env_var_or_default("S3_BUCKET", "workflow")
S3_ACCESS_KEY := env_var_or_default("S3_ACCESS_KEY", "admin")
S3_SECRET_KEY := env_var_or_default("S3_SECRET_KEY", "password")
AWS_REGION := env_var_or_default("AWS_REGION", "us-east-1")

# Show available commands
default:
    @just --list

# Prover and compress locally
prove-local:
    cargo run -r -F cuda risc0

# Prove using Bento (without Boundless)
prove-bento:
   RISC0_PROVER=bonsai BONSAI_API_URL={{BONSAI_API_URL}} BONSAI_API_KEY="" cargo run -r -F cuda risc0

# Prove by sending an order to the Boundless Market
prove-boundless:
    S3_BUCKET={{S3_BUCKET}} S3_URL={{S3_URL}} S3_ACCESS_KEY={{S3_ACCESS_KEY}} S3_SECRET_KEY={{S3_SECRET_KEY}} AWS_REGION={{AWS_REGION}} cargo run -r -F cuda boundless

prove-local-docker:
    cargo run -r risc0 