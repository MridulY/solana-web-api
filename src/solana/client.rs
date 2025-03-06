use actix_web::web;
use serde::Serialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::{fs, path::PathBuf, str::FromStr};
use borsh::BorshDeserialize;

fn get_solana_client() -> RpcClient {
    RpcClient::new("https://api.devnet.solana.com".to_string()) 
}

#[derive(BorshDeserialize, Debug, Serialize)]
pub struct Task {
    pub author: Pubkey,
    pub is_done: bool,
    pub text: String,
    pub created_at: i64,
    pub updated_at: i64,
}

const DISCRIMINATOR: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const BOOL_LENGTH: usize = 1;
const TEXT_LENGTH: usize = 4 + 400 * 4; // 400 chars
const TIMESTAMP_LENGTH: usize = 8;

impl Task {
    const LEN: usize = DISCRIMINATOR + 
        PUBLIC_KEY_LENGTH + 
        PUBLIC_KEY_LENGTH +
        BOOL_LENGTH + 
        TEXT_LENGTH +  
        TIMESTAMP_LENGTH + 
        TIMESTAMP_LENGTH; 
}

// Load Keypair from Solana CLI's id.json file
fn load_keypair() -> Result<Keypair, String> {
    let home_dir = dirs::home_dir().ok_or("Failed to find home directory")?;
    let path = home_dir.join(".config/solana/id.json");

    let keypair_bytes: Vec<u8> = serde_json::from_str(
        &fs::read_to_string(&path).map_err(|e| format!("Failed to read keypair file: {}", e))?,
    )
    .map_err(|e| format!("Invalid keypair JSON format: {}", e))?;

    Keypair::from_bytes(&keypair_bytes).map_err(|e| format!("Failed to create keypair: {}", e))
}

pub async fn add_todo_on_solana(text: &str) -> Result<(String, String), String> {
    let payer = load_keypair()?;
    let program_id = Pubkey::from_str("7AzUsuwMKP9XFpQaVt8Nt2XyAw8UHLWMYLnenxysV9Ce").unwrap();
    let task_pubkey = Keypair::new(); 

    web::block(move || {
        let rpc_client = get_solana_client();

        let rent_exempt_balance = rpc_client
            .get_minimum_balance_for_rent_exemption(1000)
            .unwrap();
        let instruction = system_instruction::create_account(
            &payer.pubkey(),
            &task_pubkey.pubkey(),
            rent_exempt_balance, 
            1000,
            &program_id,
        );

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .map_err(|e| e.to_string())?;

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer, &task_pubkey],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| e.to_string())?;

        Ok((signature.to_string(), task_pubkey.pubkey().to_string())) 
    })
    .await
    .map_err(|e| e.to_string())?
}

pub async fn update_todo_on_solana(todo_id: &str, is_done: bool) -> Result<String, String> {
    let payer = load_keypair()?;
    let task_pubkey = Pubkey::from_str(todo_id).unwrap();

    web::block(move || {
        let rpc_client = get_solana_client();

        let instruction = system_instruction::transfer(&payer.pubkey(), &task_pubkey, 0);

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .map_err(|e| e.to_string())?;

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| e.to_string())?;

        Ok(signature.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}

pub async fn fetch_task_by_id(task_id: &str) -> Result<Task, String> {
    let task_pubkey = Pubkey::from_str(task_id).map_err(|e| e.to_string())?;

    let task_result: Result<Task, String> = web::block(move || {
        let rpc_client = get_solana_client();

        let raw_data = rpc_client
            .get_account_data(&task_pubkey)
            .map_err(|e| format!("Failed to fetch task: {}", e))?;
        println!("🔹 Raw Data Length: {}", raw_data.len());
        println!("🔹 Expected Task::LEN: {}", Task::LEN);
        println!("🔹 Raw Data (Base64): {:?}", base64::encode(&raw_data));

        let trimmed_data = &raw_data[..Task::LEN.min(raw_data.len())];
        let task: Task = Task::try_from_slice(&trimmed_data)
            .map_err(|e| format!("Failed to deserialize task: {}", e))?;



        Ok(task) 
    })
    .await
    .map_err(|e| e.to_string())?;

    Ok(task_result?)
}

pub async fn delete_todo_on_solana(todo_id: &str) -> Result<String, String> {
    let payer = load_keypair()?;
    let task_pubkey = Pubkey::from_str(todo_id).unwrap();

    web::block(move || {
        let rpc_client = get_solana_client();

        let instruction = system_instruction::transfer(
            &payer.pubkey(),
            &task_pubkey,
            0,
        );

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .map_err(|e| e.to_string())?;

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| e.to_string())?;

        Ok(signature.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}
