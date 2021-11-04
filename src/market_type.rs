/// Market type.
#[repr(C)]
#[derive(Copy, Clone)]
pub enum MarketType {
    Unknown,
    Spot,
    LinearFuture,
    InverseFuture,
    LinearSwap,
    InverseSwap,

    AmericanOption,
    EuropeanOption,

    QuantoFuture,
    QuantoSwap,

    Move,
    #[allow(clippy::upper_case_acronyms)]
    BVOL,
}

impl MarketType {
    // Converts C MarketType to Rust MarketType
    pub fn to_rust(self) -> crypto_markets::MarketType {
        match self {
            MarketType::Unknown => crypto_markets::MarketType::Unknown,
            MarketType::Spot => crypto_markets::MarketType::Spot,
            MarketType::LinearFuture => crypto_markets::MarketType::LinearFuture,
            MarketType::InverseFuture => crypto_markets::MarketType::InverseFuture,
            MarketType::LinearSwap => crypto_markets::MarketType::LinearSwap,
            MarketType::InverseSwap => crypto_markets::MarketType::InverseSwap,
            MarketType::AmericanOption => crypto_markets::MarketType::AmericanOption,
            MarketType::EuropeanOption => crypto_markets::MarketType::EuropeanOption,
            MarketType::QuantoFuture => crypto_markets::MarketType::QuantoFuture,
            MarketType::QuantoSwap => crypto_markets::MarketType::QuantoSwap,
            MarketType::Move => crypto_markets::MarketType::Move,
            MarketType::BVOL => crypto_markets::MarketType::BVOL,
        }
    }

    // Converts Rust MarketType to C MarketType
    pub fn from_rust(market_type: crypto_markets::MarketType) -> Self {
        match market_type {
            crypto_markets::MarketType::Unknown => MarketType::Unknown,
            crypto_markets::MarketType::Spot => MarketType::Spot,
            crypto_markets::MarketType::LinearFuture => MarketType::LinearFuture,
            crypto_markets::MarketType::InverseFuture => MarketType::InverseFuture,
            crypto_markets::MarketType::LinearSwap => MarketType::LinearSwap,
            crypto_markets::MarketType::InverseSwap => MarketType::InverseSwap,
            crypto_markets::MarketType::AmericanOption => MarketType::AmericanOption,
            crypto_markets::MarketType::EuropeanOption => MarketType::EuropeanOption,
            crypto_markets::MarketType::QuantoFuture => MarketType::QuantoFuture,
            crypto_markets::MarketType::QuantoSwap => MarketType::QuantoSwap,
            crypto_markets::MarketType::Move => MarketType::Move,
            crypto_markets::MarketType::BVOL => MarketType::BVOL,
        }
    }
}
