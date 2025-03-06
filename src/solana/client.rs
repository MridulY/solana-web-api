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
use std::{fs, str::FromStr};
use borsh::BorshDeserialize;

// Initialize Solana RPC client
fn get_solana_client() -> RpcClient {
    RpcClient::new("https://api.devnet.solana.com".to_string()) 
}

// Define the Task structure for on-chain storage
#[derive(BorshDeserialize, Debug, Serialize)]
pub struct Task {
    pub author: Pubkey,  // Task creator
    pub is_done: bool,   // Completion status
    pub text: String,    // Task description
    pub created_at: i64, // Creation timestamp
    pub updated_at: i64, // Last update timestamp
}

// Define Task storage size
impl Task {
    const LEN: usize = 8  // Anchor's discriminator
        + 32  // Task ID
        + 32  // Author's public key
        + 1   // Completion status
        + (4 + 400 * 4)  // Task text (max 400 chars)
        + 8   // Created timestamp
        + 8;  // Updated timestamp
}

// Load keypair from Solana CLI's id.json
fn load_keypair() -> Result<Keypair, String> {
    let home_dir = dirs::home_dir().ok_or("Failed to find home directory")?;
    let path = home_dir.join(".config/solana/id.json");

    let keypair_bytes: Vec<u8> = serde_json::from_str(
        &fs::read_to_string(&path).map_err(|e| format!("Failed to read keypair file: {}", e))?,
    )
    .map_err(|e| format!("Invalid keypair JSON format: {}", e))?;

    Keypair::from_bytes(&keypair_bytes).map_err(|e| format!("Failed to create keypair: {}", e))
}

// Create a new task on Solana
pub async fn add_todo_on_solana(text: &str) -> Result<(String, String), String> {
    let payer = load_keypair()?;
    let program_id = Pubkey::from_str("7AzUsuwMKP9XFpQaVt8Nt2XyAw8UHLWMYLnenxysV9Ce").unwrap();
    let task_pubkey = Keypair::new(); 

    web::block(move || {
        let rpc_client = get_solana_client();

        let rent_exempt_balance = rpc_client
            .get_minimum_balance_for_rent_exemption(Task::LEN)
            .map_err(|e| e.to_string())?;

        let instruction = system_instruction::create_account(
            &payer.pubkey(),
            &task_pubkey.pubkey(),
            rent_exempt_balance, 
            Task::LEN as u64,
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

// Update the task status
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

// Fetch task details using task ID
pub async fn fetch_task_by_id(task_id: &str) -> Result<Task, String> {
    let task_pubkey = Pubkey::from_str(task_id).map_err(|e| e.to_string())?;

    let task_result: Result<Task, String> = web::block(move || {
        let rpc_client = get_solana_client();

        let raw_data = rpc_client
            .get_account_data(&task_pubkey)
            .map_err(|e| format!("Failed to fetch task: {}", e))?;

        // Ensure we only parse the necessary bytes
        let trimmed_data = &raw_data[..Task::LEN.min(raw_data.len())];
        let task: Task = Task::try_from_slice(trimmed_data)
            .map_err(|e| format!("Failed to deserialize task: {}", e))?;

        Ok(task) 
    })
    .await
    .map_err(|e| e.to_string())?;

    Ok(task_result?)
}

// Delete a task
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
