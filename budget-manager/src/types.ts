export type Category = {
  id: number;
  category_name: string;
  color: string;
};

export type Transaction = {
  id: number;
  trx_date: string;
  description: string;
  amount: number;
  trx_type: "CR" | "DB";
  posted_date: string | null;
  category: Category;
};

export type TransactionSummary = {
  total_income: number;
  total_expenses: number;
  total_count: number;
  transactions: Transaction[];
};
