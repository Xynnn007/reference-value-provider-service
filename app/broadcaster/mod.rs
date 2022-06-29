// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

//! Broadcaster for RVPS

pub mod cache;
pub mod as_api;

use crate::data_structs::reference_value::ReferenceValue;

use anyhow::*;

use self::{cache::CacheAPI, as_api::ASAPI};

/// BroadcasterAPI defines interfaces of Broadcaster.
pub trait BroadcasterAPI {
    /// Store the ReferenceValue into Broadcaster's
    /// Cache, and then publish it to the subscribers,
    /// e.g. the Attestation Service.
    fn store_and_publish(
        &mut self,
        rv: ReferenceValue,
    ) -> Result<()>;
}

/// Struct works as Broadcaster. `cache` is the Cache
/// which stores reference values. `as_api` is responsible
/// for communicating with Attestation Service.
pub struct Broadcaster {
    cache: Box<dyn CacheAPI + Send + Sync>,
    as_api: Box<dyn ASAPI + Send + Sync>,
}

impl BroadcasterAPI for Broadcaster {
    fn store_and_publish(
        &mut self,
        rv: ReferenceValue,
    ) -> Result<()> {
        let message = serde_json::to_string(&rv)?;
        
        // store in the Cache
        self.cache.put(rv.name(), rv)?;

        // publish
        self.as_api.publish(message)?;
        Ok(())
    }
}