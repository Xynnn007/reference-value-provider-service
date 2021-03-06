// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

//! reference value for RVPS

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

/// Default version of ReferenceValue
pub const REFERENCE_VALUE_VERSION: &str = "0.1";

/// A HashValuePair stores a hash algorithm name
/// and relative artifact's hash value due to
/// the algorithm.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HashValuePair {
    alg: String,
    value: String,
}

impl HashValuePair {
    pub fn new(alg: String, value: String) -> Self {
        Self { alg, value }
    }

    pub fn alg(&self) -> &String {
        &self.alg
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

/// Helper to deserialize an expired time
fn primitive_date_time_from_str<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<DateTime<Utc>, D::Error> {
    let s: Option<String> = Deserialize::deserialize(d)?;
    if s.is_none() {
        return Err(serde::de::Error::invalid_length(0, &"<TIME>"));
    }
    let s = s.unwrap();

    let ndt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%SZ")
        .map_err(|err| serde::de::Error::custom::<String>(err.to_string()))?;

    Ok(DateTime::<Utc>::from_utc(ndt, Utc))
}

/// Define Reference Value.
/// This Reference Value is not the same as Reference in IETF's RATS.
/// Here, RV is consumed by AS. Its format MAY be modified often to
/// cowork with AS.
/// * `version`: version of the reference value format.
/// * `name`: name of the artifact related to this reference value.
/// * `expired`: expired time for this reference value.
/// * `hash_value`: A set of key-value pairs, each indicates a hash
/// algorithm and its relative hash value for the artifact.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ReferenceValue {
    #[serde(default = "default_version")]
    version: String,
    name: String,
    #[serde(deserialize_with = "primitive_date_time_from_str")]
    expired: DateTime<Utc>,
    #[serde(rename = "hash-value")]
    hash_value: Vec<HashValuePair>,
}

/// Set the default version for ReferenceValue
fn default_version() -> String {
    REFERENCE_VALUE_VERSION.into()
}

impl ReferenceValue {
    pub fn new() -> Self {
        ReferenceValue {
            version: REFERENCE_VALUE_VERSION.into(),
            name: String::new(),
            expired: Utc::now(),
            hash_value: Vec::new(),
        }
    }

    /// Get version of the ReferenceValue.
    pub fn set_version(mut self, version: &str) -> Self {
        self.version = version.into();
        self
    }

    /// Get version of the ReferenceValue.
    pub fn version(&self) -> &String {
        &self.version
    }

    /// Get expired time of the ReferenceValue.
    pub fn set_expired(mut self, expired: DateTime<Utc>) -> Self {
        self.expired = expired;
        self
    }

    /// Get expired of the ReferenceValue.
    pub fn expired(&self) -> &DateTime<Utc> {
        &self.expired
    }

    /// Get version of the ReferenceValue.
    pub fn add_hash_value(mut self, alg: String, value: String) -> Self {
        self.hash_value.push(HashValuePair::new(alg, value));
        self
    }

    /// Get version of the ReferenceValue.
    pub fn hash_values(&self) -> &Vec<HashValuePair> {
        &self.hash_value
    }

    /// Set name for Reference Value
    pub fn set_name(mut self, name: &str) -> Self {
        self.name = name.into();
        self
    }

    /// Get artifact name of the ReferenceValue.
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod test {
    use chrono::{TimeZone, Utc};
    use serde_json::json;

    use super::ReferenceValue;

    #[test]
    fn reference_value_serialize() {
        let rv = ReferenceValue::new()
            .set_version("1.0")
            .set_name("artifact")
            .set_expired(Utc.ymd(1970, 1, 1).and_hms(0, 0, 0))
            .add_hash_value("sha512".into(), "123".into());

        assert_eq!(rv.version(), "1.0");

        let rv_json = json!({
            "expired": "1970-01-01T00:00:00Z",
            "name": "artifact",
            "version": "1.0",
            "hash-value": [{
                "alg": "sha512",
                "value": "123"
            }]
        });

        let serialized_rf = serde_json::to_value(&rv).unwrap();
        assert_eq!(serialized_rf, rv_json);
    }

    #[test]
    fn reference_value_deserialize() {
        let rv = ReferenceValue::new()
            .set_version("1.0")
            .set_name("artifact")
            .set_expired(Utc.ymd(1970, 1, 1).and_hms(0, 0, 0))
            .add_hash_value("sha512".into(), "123".into());

        assert_eq!(rv.version(), "1.0");
        let rv_json = r#"{
            "expired": "1970-01-01T00:00:00Z",
            "name": "artifact",
            "version": "1.0",
            "hash-value": [{
                "alg": "sha512",
                "value": "123"
            }]
        }"#;
        let deserialized_rf: ReferenceValue = serde_json::from_str(&rv_json).unwrap();
        assert_eq!(deserialized_rf, rv);
    }
}
