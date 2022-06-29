// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

//! AS API for Broadcaster

use anyhow::*;
use redis::Commands;

/// ASAPI contains interfaces of an `ASAPI` in RVPS.
pub trait ASAPI {
    /// Publish the message to all the subscribers
    fn publish(&mut self, message: String) -> Result<()>;
}

/// ASProxy implements ASAPI using redis as a publisher. 
/// It is responsible for communicating with Attestation Service.
/// * `conn` is the redis connection.
/// * `channel` is the redis channel for publishing.
pub struct ASProxy {
    conn: redis::Connection,
    channel: String,
}

impl ASProxy {
    /// Create a ASProxy.
    /// * `channel` is the redis channel for publishing.
    /// * `addr` is the address of redis server.
    pub fn new(channel: String, addr: String) -> Result<Self> {
        let conn = redis::Client::open(addr)?
            .get_connection()?;
        
        Ok(Self { conn, channel })
    }
}

impl ASAPI for ASProxy {
    fn publish(&mut self, message: String) -> Result<()> {
        self.conn.publish(self.channel.clone(), message)?;
        Ok(())
    }
}