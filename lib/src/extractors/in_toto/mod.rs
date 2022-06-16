use std::str::FromStr;

use anyhow::anyhow;
use in_totolib_rs::intoto::verify;

use super::Extractor;

pub struct InTotoExtractor {}

impl InTotoExtractor {
    pub fn new() -> Self {
        InTotoExtractor {}
    }
}

impl Extractor for InTotoExtractor {
    fn verify_and_extract(
        &self,
        _provenance: String,
        parameters: std::collections::HashMap<String, String>,
    ) -> anyhow::Result<String> {
        let layout_path = parameters
            .get("layout_path")
            .ok_or(anyhow!("parameters do not have layout path!"))?
            .to_owned();

        let pub_key_paths_str = parameters
            .get("pub_key_paths")
            .ok_or(anyhow!("parameters do not have pub key paths!"))?
            .to_owned();

        let pub_key_paths = serde_json::from_str(&pub_key_paths_str)?;
        
        let intermediate_paths_str = parameters
            .get("intermediate_paths")
            .ok_or(anyhow!("parameters do not have intermediate paths!"))?;

        let intermediate_paths = serde_json::from_str(&intermediate_paths_str)?;
        
        let link_dir = parameters
            .get("link_dir")
            .ok_or(anyhow!("parameters do not have link files dir path!"))?
            .to_owned();

        let line_normalization_str = parameters
            .get("line_normalization")
            .ok_or(anyhow!("parameters do not have line normalization!"))?;
        
        let line_normalization = FromStr::from_str(line_normalization_str)?;

        // Here the returned value is "" when verification successeds
        let _ = verify(layout_path, pub_key_paths, intermediate_paths, link_dir, line_normalization)?;

        // Up to now, just verify the in-toto provenance
        // But need to extract the artifact's hash value, which is not implemented now 
        // TODO
        Err(anyhow!("Can not extract hash value using in-toto"))
    }
}