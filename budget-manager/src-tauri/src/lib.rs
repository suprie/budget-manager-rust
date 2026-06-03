// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::AppHandle;
use statement_core:: TransactionLine;

mod db;
mod transactions;

#[tauri::command]
fn read_statement_file(app_handle: AppHandle, filepath: String) -> Result<String, String> {
    let transactions =
        bca_pdf::parse_pdf_file_to_statement(&filepath).map_err(|error| error.to_string())?;
    insert_transactions(app_handle, &transactions.transactions)
}

#[tauri::command]
fn read_csv_statement_file(app_handle: AppHandle, filepath: String, year: i32) -> Result<String, String> {
    let transactions =
        bca_csv::parse_csv_file(&filepath, year).map_err(|error| error.to_string())?;
    insert_transactions(app_handle, &transactions)
}

#[tauri::command]
fn reset_db(app_handle: AppHandle) {
    let _ = db::reset_db(app_handle);
}

#[tauri::command]
fn load_transactions(app_handle: AppHandle) -> Result<transactions::TransactionSummary, String> {
   let mut conn = db::open_database(app_handle)?;
   let result = transactions::load_data(&mut conn)?;

    Ok(result)
}

#[tauri::command]
fn list_transactions(app_handle: AppHandle) -> Result<Vec<transactions::StoredTransaction>, String> {
   let mut conn = db::open_database(app_handle)?;
   let result = transactions::read_all_transactions(&mut conn)?;

   println!("Result {:?}",result);
   Ok(result)
}

fn insert_transactions(app_handle: AppHandle, transactions: &[TransactionLine]) -> Result<String, String> {
    let mut conn = db::open_database(app_handle)?;
    let size = transactions::insert_transactions(&mut conn, transactions)?;
    Ok(format!("Transaction Size {:?}",  size))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            read_statement_file,
            read_csv_statement_file,
            reset_db,
            list_transactions,
            load_transactions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
