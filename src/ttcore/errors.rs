// @todo: add information for each exception type, need to know types

#[derive(Debug)]
pub enum TensorTradeError {
    // QUANTITY EXCEPTIONS
    /// Raised when a `Quantity` tries to be instantiated with a negative amount
    InvalidNegativeQuantity {
        size: f32,
    },

    /// Raised when a `Quantity` tries to be instantiated with a value that is not numeric
    InvalidNonNumericQuantity {},

    /// Raised when an operation tries to occur between quantities that are not under the same
    /// path_id.
    QuantityOpPathMismatch {},

    /// Raised when a locked `Quantity` is trying to get locked again
    DoubleLockedQuantity {},

    /// Raised when a free `Quantity` is trying to get unlocked
    DoubleUnlockedQuantity {},

    /// Raised when a locked `Quantity` does not have a path_id in the `Wallet` it is trying to be
    /// unlocked in.
    QuantityNotLocked {},

    // INSTRUMENT EXCEPTIONS
    /// Raised when two quantities with different instruments occurs
    IncompatibleInstrumentOperation {},

    // ORDER EXCEPTIONS
    /// Raised when an `Order` with a non-negative amount is placed
    InvalidOrderQuantity {},
    DoubleCancelOrder {},

    // WALLET EXCEPTIONS
    /// Raised when requested funds are greater than the free balance of a `Wallet`
    InsufficientFunds {},
    WalletNotFound {},

    // TRADING PAIR EXCEPTIONS
    /// Raised when an invalid trading pair is trying to be created
    InvalidTradingPair {},
}
