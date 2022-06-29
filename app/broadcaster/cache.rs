// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

//! Cache for Broadcaster

use std::collections::HashMap;

use crate::data_structs::reference_value::ReferenceValue;

use anyhow::*;

/// CacheAPI defines interfaces of Cache
pub trait CacheAPI {
    /// Put an Reference Value into the Cache.
    fn put(&mut self, artifact_name: String, reference_value: ReferenceValue) -> Result<()>;
    /// Get all the Reference Values from the Cache.
    fn get_all(&self) -> Result<Vec<ReferenceValue>>;
    // fn Revoke(&mut self, reference_value: String) -> Result<()>;
}

/// An Cache will store reference values.
pub struct Cache {
    inner: HashMap<String, ReferenceValue>,
}

impl CacheAPI for Cache {
    fn put(&mut self, artifact_name: String, reference_value: ReferenceValue) -> Result<()> {
        self.inner.insert(artifact_name, reference_value);
        Ok(())
    }

    fn get_all(&self) -> Result<Vec<ReferenceValue>> {
        let res = self.inner
            .iter()
            .map(|kv| {
                (*(kv.1)).clone()
            })
            .collect();
        
        Ok(res)
    }
}
