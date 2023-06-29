use anyhow::{Context, Result};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sts::{config::Region, Client};
use clap::{Parser, ValueEnum};
use serde_json::json;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputType {
    /// Output as json
    Json,
    /// Output as regular string
    String,
}

#[derive(Debug, Parser)]
struct Opt {
    /// The AWS Region.
    #[clap(short, long)]
    region: Option<String>,

    /// Which profile to use.
    #[clap(short, long)]
    profile: Option<String>,

    #[arg(value_enum)]
    #[arg(default_value_t=OutputType::String)]
    #[clap(short, long)]
    output_type: OutputType,
}

async fn get_caller_identity(client: &Client, output_type: OutputType) -> Result<()> {
    let response = client
        .get_caller_identity()
        .send()
        .await
        .with_context(|| "Failed loading AWS config details. Did you run 'aws configure' ?")?;

    let account = response.account().unwrap_or_default();
    let account_arn = response.account().unwrap_or_default();
    let user_id = response.user_id().unwrap_or_default();
    
    match output_type {
        OutputType::String => {
            println!("AccountId = {}", account);
            println!("AccountArn = {}", account_arn);
            println!("UserID = {}", user_id);
        }
        OutputType::Json => {
            let result = json!({
                "accountId": account,
                "AccountArn": account_arn,
                "UserID": user_id,
            });
            println!("{}", result.to_string());
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Opt::parse();

    let region_provider = RegionProviderChain::first_try(args.region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    println!();

    let shared_config = if let Some(p) = args.profile {
        aws_config::from_env().region(region_provider).profile_name(p).load().await
    } else {
        aws_config::from_env().region(region_provider).load().await
    };
    
    let client = Client::new(&shared_config);
    get_caller_identity(&client, args.output_type).await
}
