use crate::oms::instruments::Quantity;

pub struct Transaction {
    poid: String,
    step: i32,
    source: String,
    target: String,
    memo: String,
    amount: Quantity,
    free: Quantity,
    locked: Quantity,
    locked_poid: Option<Quantity>,
}

pub trait WalletLike {
    fn balance<'a>(&'a self) -> &'a Quantity;
    fn is_locked(&self, key: &String) -> bool;
    fn locked(&self, key: &String) -> Option<Quantity>;
    fn step(&self) -> i32;
    fn locked_balance(&self) -> Quantity;
}

pub struct Ledger {
    transactions: Vec<Transaction>,
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger {
            transactions: Vec::new(),
        }
    }

    pub fn commit(
        &mut self,
        wallet: Box<&dyn WalletLike>,
        quantity: Quantity,
        source: String,
        target: String,
        memo: String,
    ) {
        let poid = quantity.path_id.clone();
        let locked_poid_balance = if wallet.is_locked(&poid) {
            None
        } else {
            wallet.locked(&poid)
        };

        let transaction = Transaction {
            poid,
            step: wallet.step(),
            source,
            target,
            memo,
            amount: quantity,
            free: wallet.balance().clone(),
            locked: wallet.locked_balance(),
            locked_poid: locked_poid_balance,
        };

        self.transactions.push(transaction);
    }
}
