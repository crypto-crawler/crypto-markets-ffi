#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use crypto_market_type::MarketType;

/// Fetch trading markets of a cryptocurrency exchange.
#[no_mangle]
pub extern "C" fn fetch_markets(exchange: *const c_char, market_type: MarketType) -> *const c_char {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };

    let result = std::panic::catch_unwind(|| {
        if let Ok(markets) = crypto_markets::fetch_markets(exchange_rust, market_type) {
            let text = serde_json::to_string(&markets).unwrap();
            let raw = CString::new(text).unwrap();
            raw.into_raw() as *const c_char
        } else {
            std::ptr::null()
        }
    });
    match result {
        Ok(ptr) => ptr,
        Err(err) => {
            eprintln!("{:?}", err);
            std::ptr::null()
        }
    }
}

/// Deallocate a string.
#[no_mangle]
pub extern "C" fn deallocate_string(pointer: *const c_char) {
    unsafe {
        if pointer.is_null() {
            return;
        }
        CString::from_raw(pointer as *mut c_char)
    };
}

#[cfg(test)]
mod tests {
    use crypto_market_type::MarketType;

    use super::{deallocate_string, fetch_markets};
    use std::ffi::{CStr, CString};

    #[test]
    fn test_fetch_markets() {
        let (json_ptr, json_str) = {
            let exchange = CString::new("binance").unwrap();
            let json_ptr = fetch_markets(exchange.as_ptr(), MarketType::InverseSwap);
            let json_c_str = unsafe {
                debug_assert!(!json_ptr.is_null());
                CStr::from_ptr(json_ptr)
            };

            (json_ptr, json_c_str.to_str().unwrap())
        };

        let markets = serde_json::from_str::<Vec<crypto_markets::Market>>(json_str).unwrap();
        assert!(!markets.is_empty());
        let market = &markets[0];

        assert_eq!(market.exchange, "binance");
        assert_eq!(market.market_type, MarketType::InverseSwap);

        deallocate_string(json_ptr);
    }
}
