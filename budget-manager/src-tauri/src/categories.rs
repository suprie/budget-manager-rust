use rusqlite::{params, Connection};

#[derive(Debug, serde::Serialize, Clone)]
pub struct Category {
    pub id: i64,
    pub category_name: String,
    pub color: String,
}

#[derive(Debug, serde::Serialize)]
pub struct UncategorizedTransaction {
    pub id: i64,
    pub trx_date: String,
    pub description: String,
    pub amount: f64,
    pub trx_type: String,
    pub posted_date: Option<String>,
    pub source: String,
    pub category: Category,
}

/// Return all categories, ordered by id.
pub fn get_all_categories(conn: &Connection) -> Result<Vec<Category>, String> {
    let mut stmt = conn
        .prepare("SELECT id, category_name, color FROM categories ORDER BY id")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Category {
                id: row.get("id")?,
                category_name: row.get("category_name")?,
                color: row.get("color")?,
            })
        })
        .map_err(|e| e.to_string())?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

/// Insert a new category and return it with its generated id.
pub fn create_category(
    conn: &Connection,
    name: &str,
    color: &str,
) -> Result<Category, String> {
    conn.execute(
        "INSERT INTO categories (category_name, color) VALUES (?1, ?2)",
        params![name, color],
    )
    .map_err(|e| e.to_string())?;

    let id = conn.last_insert_rowid();

    Ok(Category {
        id,
        category_name: name.to_string(),
        color: color.to_string(),
    })
}

/// Fetch transactions assigned to the 'unknown' category (id = 1).
/// If `keyword` is non-empty, filter by description LIKE.
pub fn get_uncategorized_transactions(
    conn: &Connection,
    keyword: &str,
) -> Result<Vec<UncategorizedTransaction>, String> {
    let sql = if keyword.is_empty() {
        "SELECT t.id, t.trx_date, t.description, t.amount, t.trx_type,
                t.posted_date, t.source,
                c.id AS cat_id, c.category_name, c.color
         FROM transactions t
         LEFT JOIN categories c ON t.category_id = c.id
         WHERE t.category_id = 1
         ORDER BY t.trx_date DESC"
            .to_string()
    } else {
        format!(
            "SELECT t.id, t.trx_date, t.description, t.amount, t.trx_type,
                    t.posted_date, t.source,
                    c.id AS cat_id, c.category_name, c.color
             FROM transactions t
             LEFT JOIN categories c ON t.category_id = c.id
             WHERE t.category_id = 1
               AND t.description LIKE '%' || ?1 || '%'
             ORDER BY t.trx_date DESC"
        )
    };

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = if keyword.is_empty() {
        stmt.query_map([], row_to_uncategorized)
            .map_err(|e| e.to_string())?
    } else {
        stmt.query_map(params![keyword], row_to_uncategorized)
            .map_err(|e| e.to_string())?
    };

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())
}

/// Assign a list of transaction ids to a category.
pub fn bulk_assign_category(
    conn: &Connection,
    transaction_ids: &[i64],
    category_id: i64,
) -> Result<usize, String> {
    if transaction_ids.is_empty() {
        return Ok(0);
    }

    // Build  "UPDATE transactions SET category_id = ?1 WHERE id IN (?2, ?3, ...)"
    let placeholders: Vec<String> = transaction_ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("?{}", i + 2))
        .collect();

    let sql = format!(
        "UPDATE transactions SET category_id = ?1 WHERE id IN ({})",
        placeholders.join(", ")
    );

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    param_values.push(Box::new(category_id));
    for id in transaction_ids {
        param_values.push(Box::new(*id));
    }

    let params_refs: Vec<&dyn rusqlite::types::ToSql> = param_values.iter().map(|p| p.as_ref()).collect();

    stmt.execute(params_refs.as_slice())
        .map_err(|e| e.to_string())
}

fn row_to_uncategorized(
    row: &rusqlite::Row<'_>,
) -> rusqlite::Result<UncategorizedTransaction> {
    Ok(UncategorizedTransaction {
        id: row.get("id")?,
        trx_date: row.get("trx_date")?,
        description: row.get("description")?,
        amount: row.get("amount")?,
        trx_type: row.get("trx_type")?,
        posted_date: row.get("posted_date")?,
        source: row.get("source")?,
        category: Category {
            id: row.get("cat_id")?,
            category_name: row.get("category_name")?,
            color: row.get("color")?,
        },
    })
}
