// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::path::Path;
use tauri::AppHandle;
use statement_core:: { TransactionLine, TrxType };

mod db;
mod transactions;

#[tauri::command]
fn read_statement_file(app_handle: AppHandle, filepath: String) -> Result<String, String> {
    let extension = Path::new(&filepath)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_lowercase())
        .ok_or_else(|| "Selected file has no extension".to_string())?;
    match extension.as_str() {
        "csv" => {
            let transactions = bca_csv::parse_csv_file(&filepath).map_err(|error| error.to_string())?;
            println!("tranactions {:?}",transactions);
            insert_transactions(app_handle, &transactions)
        }
        "pdf" => {
            let transactions =
                bca_pdf::parse_pdf_file(&filepath).map_err(|error| error.to_string())?;
            insert_transactions(app_handle, &transactions)
        }
        _ => Err(format!("Unsupported statement file type: {extension}")),
    }
}

fn insert_transactions(app_handle: AppHandle, transactions: &Vec<TransactionLine>) -> Result<String, String> {
    let mut conn = db::open_database(app_handle)?;
    let size = transactions::insert_transactions(&mut conn, &transactions)?;
    Ok(format!("Transaction Size {:?}",  size))
}

#[tauri::command]
fn reset_db(app_handle: AppHandle) {
    let _ = db::reset_db(app_handle);
}

#[tauri::command]
fn list_transactions(app_handle: AppHandle) -> Result<Vec<transactions::StoredTransaction>, String> {
   let mut conn = db::open_database(app_handle)?;
   let result = transactions::read_all_transactions(&mut conn)?;

   println!("Result {:?}",result);
   return Ok(result)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            read_statement_file,
            reset_db,
            list_transactions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
