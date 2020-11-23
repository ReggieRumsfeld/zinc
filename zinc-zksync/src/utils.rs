//!
//! The array-vector conversion tools.
//!

use zksync::web3::types::Address;
use zksync::web3::types::H256;

///
/// Converts the `BigUint` v0.3 to `BigUint` v0.2.
///
pub fn num_compat_backward(value: num::BigUint) -> num_old::BigUint {
    num_old::BigUint::from_bytes_be(value.to_bytes_be().as_slice())
}

///
/// Converts the `BigUint` v0.2 to `BigUint` v0.3.
///
pub fn num_compat_forward(value: num_old::BigUint) -> num::BigUint {
    num::BigUint::from_bytes_be(value.to_bytes_be().as_slice())
}

///
/// Converts a slice into an ETH address.
///
/// # Panics
/// If the `slice` size is less than that of the ETH address.
///
pub fn address_from_slice(slice: &[u8]) -> Address {
    let mut array = [0; zinc_const::size::ETH_ADDRESS];
    for (index, byte) in slice.iter().enumerate() {
        array[index] = *byte;
    }
    array.into()
}

///
/// Converts a slice into an ETH private key.
///
/// # Panics
/// If the `slice` size is less than that of the ETH private key.
///
pub fn private_key_from_slice(slice: &[u8]) -> H256 {
    let mut array = [0; zinc_const::size::ETH_PRIVATE_KEY];
    for (index, byte) in slice.iter().enumerate() {
        array[index] = *byte;
    }
    array.into()
}
