pub trait Storekit2 {

    /// verify jws
    fn verified() -> bool;

    /// decode jws string to struct
    fn decode_jws();
}

enum JWSString {

    /// JWS format string of transaction
    Transaction(String),

    /// JWS format string of subscription renewal info
    RenewalInfo(String),
}

pub enum OrderLookupStatus {
    OrderIDValid = 0,
    OrderIDInValid = 1,
}

/// [web-page](https://developer.apple.com/documentation/appstoreserverapi/jwsdecodedheader)
pub struct JWSDecodedHeader {
    alg: String,
    kid: String,
    x5c: Vec<String>,
}

impl JWSDecodedHeader {
    fn new(alg: String, kid: String, x5c: Vec<String>) -> Self {
        Self{
            alg,
            kid,
            x5c,
        }
    }
    fn get_publickey() {
        unimplemented!();
    }
}

pub struct BadRequest {
    error_code: i64,
    error_message: String,
}

impl BadRequest {
    fn new(error_code: i64, error_message: String) -> Self{
        Self{
            error_code,
            error_message,
        }
    }

    pub fn get_error_code(&self) -> i64 {
        self.error_code
    }

    pub fn get_error_message(&self) -> &str {
        &self.error_message
    }
}


/// [web-page](https://developer.apple.com/documentation/appstoreserverapi/orderlookupresponse)
pub struct OrderLookupResponse {
    /// The status that indicates whether the order ID is valid.
    pub status: OrderLookupStatus,
    /// An array of in-app purchase transactions that are part of order,
    /// signed by Apple, in JSON Web Signature format.
    signed_transactions: Vec<JWSString>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sad_bad_request() {
        let bad_request = BadRequest::new(123, String::from("test"));
        assert_eq!(123, bad_request.get_error_code());
    }
}
