// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::AppHandle;
use statement_core:: TransactionLine;

mod categories;
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
fn load_transactions(
    app_handle: AppHandle,
    limit: i64,
    offset: i64,
) -> Result<transactions::TransactionSummary, String> {
   let mut conn = db::open_database(app_handle)?;
   let result = transactions::load_data(&mut conn, limit, offset)?;

    Ok(result)
}

#[tauri::command]
fn get_all_categories(app_handle: AppHandle) -> Result<Vec<categories::Category>, String> {
    let mut conn = db::open_database(app_handle)?;
    categories::get_all_categories(&mut conn)
}

#[tauri::command]
fn create_category(
    app_handle: AppHandle,
    category_name: String,
    color: String,
) -> Result<categories::Category, String> {
    let mut conn = db::open_database(app_handle)?;
    categories::create_category(&mut conn, &category_name, &color)
}

#[tauri::command]
fn get_uncategorized_transactions(
    app_handle: AppHandle,
    keyword: String,
    limit: i64,
    offset: i64,
) -> Result<Vec<categories::UncategorizedTransaction>, String> {
    let mut conn = db::open_database(app_handle)?;
    categories::get_uncategorized_transactions(&mut conn, &keyword, limit, offset)
}

#[tauri::command]
fn bulk_assign_category(
    app_handle: AppHandle,
    transaction_ids: Vec<i64>,
    category_id: i64,
) -> Result<usize, String> {
    let mut conn = db::open_database(app_handle)?;
    categories::bulk_assign_category(&mut conn, &transaction_ids, category_id)
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
            load_transactions,
            get_all_categories,
            create_category,
            get_uncategorized_transactions,
            bulk_assign_category
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
