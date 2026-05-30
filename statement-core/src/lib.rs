
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
}


