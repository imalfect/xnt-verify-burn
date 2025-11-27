use neptune_cash::api::export::TransparentTransactionInfo;
use neptune_cash::application::json_rpc::core::api::rpc::RpcApi;
use neptune_cash::prelude::triton_vm::prelude::triton_program;
use neptune_cash::protocol::consensus::block::block_selector::BlockSelector;
use neptune_cash::protocol::consensus::transaction::lock_script::LockScript;
use neptune_rpc_client::http::HttpClient;
use std::env;

#[tokio::main]
async fn main() {
    println!("--- XNT Burn Prover ---");
    const BURN_ERROR: i128 = 1_000_300;
    let burn_lock_script = LockScript::new(triton_program! {
        push 0 assert error_id {BURN_ERROR}
    });

    let rpc_url = env::var("NPT_RPC_URL").unwrap_or("http://217.160.149.196:9797".to_string());

    let client = HttpClient::new(rpc_url);
    let tip = client.height().await.unwrap();
    println!(
        "Connected to Neptune Cash RPC, current chain height: {}",
        tip.height.value()
    );

    println!("Fetching block 16999 announcements to find burn transaction");
    let announcements = client
        .get_block_announcements(BlockSelector::Height(16999.into()))
        .await
        .unwrap()
        .announcements
        .unwrap();
    println!("Block 16999 has {} announcements", announcements.len());

    // index 4 announcement in this block is the burn one
    let announcement = &announcements[4];
    let tti =
        TransparentTransactionInfo::try_from_announcement(&announcement.clone().into()).unwrap();
    println!(
        "Fetched the transaction, has {} outputs, checking",
        tti.outputs.len()
    );
    tti.outputs.iter().for_each(|output| {
        // check if lock script of that tx is the burn one
        let output_native_currency_amount = output.utxo.get_native_currency_amount().ceil_num_whole_coins();
        if (output.utxo.lock_script_hash() == burn_lock_script.hash()) {
            println!(
                "Found output with {:?} coins, this IS a burn output!",
                output_native_currency_amount

            );
        } else {
            println!(
                "Found output with {:?} coins, this IS NOT a burn output!",
                output_native_currency_amount
            );
        }
    });
}
