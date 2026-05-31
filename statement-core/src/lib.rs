use chrono::NaiveDate;

#[derive(Debug)]
pub struct SummaryTransaction {
    pub account_number: String,
    pub period: String,
    pub transactions: Vec<TransactionLine>
}

#[derive(Debug, PartialEq)]
pub enum TrxType {
    CR,
    DB,
}

#[derive(Debug)]
pub struct TransactionLine {
    pub trx_date: String,
    pub description: String,
    pub amount: f64,
    pub trx_type: TrxType,
    pub posted_date: Option<NaiveDate>
}


