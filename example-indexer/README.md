# nearsocial-indexer

## Development

### Prerequsites

1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install `diesel-cli`

```bash
$ cargo install diesel_cli --no-default-features --features "postgres"
```

3. Create a PostgreSQL database

4. Add `.env` file with the parameters you need

```
DATABASE_URL=postgres://user:pass@host/databasename
CHAIN_ID=testnet
START_BLOCK_HEIGHT=105141584 # the block when near social was deployed on testnet
WHITELIST_ACCOUNTS=social.near,v1.social08.testnet
RUST_LOGS="nearsocial-indexer=info,near-lake-framework=info"

AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE
AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
```

**Please note** the `RUST_LOGS`. You can ommit it and indexer is going to run silently.

5. Apply migrations

```bash
$ diesel migration run
```

6. Build the project

```bash
$ cargo build --release
```

7. Run

```bash
./target/release/nearsocial-indexer
```

7.1 AWS credentials

Don't forget to provide AWS credentials that can read from S3 buckets (using Requester Pays). You can provide them either via file or using environmental variables. Ref: https://docs.rs/near-lake-framework/0.7.1/near_lake_framework/#file-based-aws-credentials

8. Stop it with `CTRL+C` when needed.

**Keep in mind** if you restart the indexer it will start from the same `START_BLOCK_HEIGHT` so consider changing this parameter before the next run or provide it during the run like this:

```bash
$ env START_BLOCK_HEIGHT=105141584 ./target/release/nearsocial-indexer
```

Happy hacking!
