//!
//! The contract resource POST call method module.
//!

pub mod error;
pub mod request;

use std::sync::Arc;
use std::sync::RwLock;

use actix_web::http::StatusCode;
use actix_web::web;
use actix_web::Responder;
use serde_json::Value as JsonValue;

use zinc_build::Value as BuildValue;
use zinc_vm::Bn256;

use crate::database::model::field::select::input::Input as FieldSelectInput;
use crate::database::model::field::select::output::Output as FieldSelectOutput;
use crate::database::model::field::update::input::Input as FieldUpdateInput;
use crate::response::Response;
use crate::shared_data::SharedData;

use self::error::Error;
use self::request::Body;
use self::request::Query;

///
/// The HTTP request handler.
///
pub async fn handle(
    app_data: web::Data<Arc<RwLock<SharedData>>>,
    query: web::Query<Query>,
    body: web::Json<Body>,
) -> impl Responder {
    let query = query.into_inner();
    let body = body.into_inner();

    let contract = match app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .contracts
        .get(&query.contract_id)
        .cloned()
    {
        Some(contract) => contract,
        None => return Response::error(Error::ContractNotFound),
    };

    let method = match contract.build.methods.get(query.method.as_str()).cloned() {
        Some(method) => method,
        None => return Response::error(Error::MethodNotFound),
    };
    if !method.is_mutable {
        return Response::error(Error::MethodIsImmutable);
    }

    let input_value = match BuildValue::try_from_typed_json(body.arguments, method.input) {
        Ok(input_value) => input_value,
        Err(error) => return Response::error(Error::InvalidInput(error)),
    };

    let storage_fields_count = contract.build.storage.len();
    let storage_value = match app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .postgresql_client
        .select_fields(FieldSelectInput::new(query.contract_id))
        .await
    {
        Ok(output) => {
            if output.len() != contract.build.storage.len() {
                return Response::error(Error::InvalidStorageSize {
                    expected: contract.build.storage.len(),
                    found: output.len(),
                });
            }

            let mut fields = Vec::with_capacity(output.len());
            for (index, FieldSelectOutput { name, value }) in output.into_iter().enumerate() {
                let r#type = contract.build.storage[index].1.clone();
                let value = match BuildValue::try_from_typed_json(value, r#type) {
                    Ok(value) => value,
                    Err(error) => return Response::error(Error::InvalidStorage(error)),
                };
                fields.push((name, value))
            }
            BuildValue::Contract(fields)
        }
        Err(error) => return Response::error(Error::Database(error)),
    };

    let build = contract.build;
    let method = query.method;
    let output = match async_std::task::spawn_blocking(move || {
        zinc_vm::ContractFacade::new(build).run::<Bn256>(input_value, storage_value, method)
    })
    .await
    {
        Ok(output) => output,
        Err(error) => return Response::error(Error::RuntimeError(error)),
    };

    let mut storage_fields = Vec::with_capacity(storage_fields_count);
    match output.storage {
        BuildValue::Contract(fields) => {
            for (index, (_name, value)) in fields.into_iter().enumerate() {
                let value = value.into_json();
                storage_fields.push(FieldUpdateInput::new(
                    index as i16,
                    query.contract_id,
                    value,
                ));
            }
        }
        _ => panic!(zinc_const::panic::VALIDATED_DURING_RUNTIME_EXECUTION),
    }

    if let Err(error) = app_data
        .read()
        .expect(zinc_const::panic::MULTI_THREADING)
        .postgresql_client
        .update_fields(storage_fields)
        .await
    {
        return Response::error(Error::Database(error));
    }

    let wallet_credentials = match zksync::WalletCredentials::from_eth_pk(
        contract.eth_address.into(),
        contract.private_key.into(),
    ) {
        Ok(wallet_credentials) => wallet_credentials,
        Err(error) => return Response::error(Error::ZkSync(error)),
    };

    let network = match query.network {
        zinc_data::Network::Localhost => zksync::Network::Localhost,
        zinc_data::Network::Rinkeby => zksync::Network::Rinkeby,
        zinc_data::Network::Ropsten => zksync::Network::Ropsten,
    };
    let provider = zksync::Provider::new(network);

    let wallet = match zksync::Wallet::new(provider, wallet_credentials).await {
        Ok(wallet) => wallet,
        Err(error) => return Response::error(Error::ZkSync(error)),
    };

    dbg!(output.transfers);

    Response::<JsonValue, Error>::success_with_data(StatusCode::OK, output.result.into_json())
}
