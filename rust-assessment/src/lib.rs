mod programs;

#[cfg(test)]
mod tests {
    use crate::programs::Turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram};
    use solana_client::rpc_client::RpcClient;
    use solana_program::system_program;
    use solana_sdk::signature::{read_keypair_file, Signer};

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn submit_course_completion() {
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Load wallet
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

        // Derive PDA
        let prereq = Turbin3PrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        // Define instruction arguments
        let args = CompleteArgs {
            github: b"parikshitlfj12".to_vec(),
        };

        // Get recent blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Create the transaction
        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print the transaction signature
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }
}
