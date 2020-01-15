///////////////////////////////////////////////////////////////////////////////
//
//  Copyright 2018-2019 Airalab <research@aira.life> 
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//
///////////////////////////////////////////////////////////////////////////////
///! Substrate API in ROS namespace.

pub use sc_rpc::system::helpers::SystemInfo;
use sc_network::{
    specialization::NetworkSpecialization,
    NetworkService, ExHashT
};
use sp_core::{H256, traits::BareCryptoStorePtr};
use sp_transaction_pool::TransactionPool;
use sp_blockchain::Error as ClientError;
use futures::future::{Future, FutureExt, join};
use sp_api::ProvideRuntimeApi;
use rosrust::api::error::Error;
use sp_session::SessionKeys;
use sp_runtime::traits;
use sc_client::Client;
use std::sync::Arc;

pub mod author;
pub mod system;
pub mod chain;
pub mod state;

pub fn start<B, E, RA, P, S, H>(
    system_info: SystemInfo,
    service_client: Arc<Client<B, E, P::Block, RA>>,
    service_network: Arc<NetworkService<P::Block, S, H>>,
    service_transaction_pool: Arc<P>,
    service_keystore: BareCryptoStorePtr,
) -> Result<(Vec<rosrust::Service>, impl Future<Output=()>), Error> where
	B: sc_client_api::backend::Backend<<P as TransactionPool>::Block> + Send + Sync + 'static,
	E: sc_client_api::CallExecutor<<P as TransactionPool>::Block> + Clone + Send + Sync + 'static,
	P: TransactionPool<Hash=H256> + Sync + Send + 'static,
	RA: Send + Sync + 'static,
	P::Block: traits::Block<Hash=H256>,
	Client<B, E, P::Block, RA>: ProvideRuntimeApi<P::Block>,
	<Client<B, E, P::Block, RA> as ProvideRuntimeApi<P::Block>>::Api:
		SessionKeys<P::Block, Error = ClientError>,
    S: NetworkSpecialization<P::Block>,
    H: ExHashT + Clone + Sync,
    u64: From<<<P::Block as traits::Block>::Header as traits::Header>::Number>,
{

    let system = system::System::new(
        system_info,
        service_network,
    );

    let author = author::Author::new(
        service_client.clone(),
        service_transaction_pool,
        service_keystore,
    );

    let chain = chain::FullChain::new(
        service_client.clone(),
    );

    let state = state::FullState::new(
        service_client,
    );

    let publishers = join(
        system::start_publishers(system.clone())?,
        chain::start_publishers(chain.clone())?,
    ).map(|_| ());

    let services = vec![
        system::start_services(system)?,
        author::start_services(author)?,
        state::start_services(state)?,
        chain::start_services(chain)?,
    ].concat();

    Ok((services, publishers))
}