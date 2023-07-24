#![allow(missing_docs)]
use sov_modules_api::AddressBech32;
use sov_modules_macros::rpc_gen;
use sov_state::WorkingSet;

use crate::{Account, Accounts};

/// This is the response returned from the accounts_getAccount endpoint.
#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Response {
    /// The account corresponding to the given public key exists.
    AccountExists {
        /// The address of the account,
        addr: AddressBech32,
        /// The nonce of the account.
        nonce: u64,
    },
    /// The account corresponding to the given public key does not exist.
    AccountEmpty,
}

#[rpc_gen(client, server, namespace = "accounts")]
impl<C: sov_modules_api::Context> Accounts<C> {
    #[rpc_method(name = "getAccount")]
    pub fn get_account(
        &self,
        pub_key: C::PublicKey,
        working_set: &mut WorkingSet<C::Storage>,
    ) -> Response {
        match self.accounts.get(&pub_key, working_set) {
            Some(Account { addr, nonce }) => Response::AccountExists {
                addr: addr.into(),
                nonce,
            },
            None => Response::AccountEmpty,
        }
    }
}
