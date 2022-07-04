// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::constant::{NETWORK, NETWORK_ZENOH};
use crate::traits::{TomlWriter, YmlWriter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PeerConfig {
    pub protocol: String,
    pub port: u16,
    pub domain: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZenohConfig {
    pub grpc_port: Option<u16>,
    pub domain: Option<String>,
    pub protocol: Option<String>,
    pub port: Option<u16>,

    pub ca_cert: Option<String>,

    pub cert: Option<String>,

    pub priv_key: Option<String>,

    #[serde(default)]
    // https://github.com/alexcrichton/toml-rs/issues/258
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub peers: Vec<PeerConfig>,

    pub node_address: Option<String>,
    pub validator_address: Option<String>,
    pub chain_id: Option<String>,
}
impl ZenohConfig {
    pub fn new(
        port: u16,
        grpc_port: u16,
        ca_cert: String,
        cert: String,
        priv_key: String,
        peers: Vec<PeerConfig>,
        domain: String,
        protocol: String,
        node_address: String,
        validator_address: String,
        chain_id: String,
    ) -> Self {
        Self {
            port: Some(port),
            grpc_port: Some(grpc_port),
            ca_cert: Some(ca_cert),
            cert: Some(cert),
            priv_key: Some(priv_key),
            peers,
            domain: Some(domain),
            protocol: Some(protocol),
            node_address: Some(node_address),
            validator_address: Some(validator_address),
            chain_id: Some(chain_id),
        }
    }
}

impl TomlWriter for ZenohConfig {
    fn section(&self) -> String {
        NETWORK_ZENOH.to_string()
    }
}

impl YmlWriter for ZenohConfig {
    fn service(&self) -> String {
        NETWORK.to_string()
    }
}
