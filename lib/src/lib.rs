// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use std::{collections::HashMap, env, path::Path};

use anyhow::{anyhow, Result};
use extractors::{ExtractorModuleList, ExtractorInstance};

mod extractors;

static WORKING_DIR_KEY: &str = "working_dir";

/// `Extractors` is the core module of Reference Value Providing Service
/// (RVPS for short). It provides different kinds of `Extractor`s due to
/// different provenance types, e.g. in-toto, etc.
/// Each `Extractor` will process the input provenance, verify the 
/// validation of the provenance, and then extract the formatted 
/// reference value (degest, s.t. hash value and name of the artifact) 
/// from the provenance. If the verification fails, no reference value 
/// will be extracted. 

/// Define an universal Reference Value
#[derive(PartialEq)]
pub struct ReferenceValue {
    provenance_name: String,
    hash_value: String,
}

/// `ExtratorsAPI` defines the interfaces of Extractors.
pub trait ExtratorsAPI {
    /// Handle the provenance, e.g. verifying and extracting.
    /// * `provenance_type` indicates the type of the provenance, e.g.
    /// `in-toto`.
    /// * `provenance_name` is the artifact's name, which will be 
    /// included in the outcome `ReferenceValue` struct.
    /// * `provenance` is the content of provenance file.
    /// * `parameters` is a k-v map. It is useful when many extra
    /// parameters are needed when processing provenance. Dfferent 
    /// provenance has different format. For example,
    /// in in-toto, provenance files include linkfiles, layoutfile and
    /// public key files. It is hard to be included all of them in 
    /// parameter `provenance`. However, if we store some strings 
    /// that indicate the paths for the files, it will be easy to
    /// handle, because all in-toto provenance has layout, linkfile and
    /// pubkeys. 
    fn handle_provenance(
        &mut self,
        provenance_type: String,
        provenance_name: String,
        provenance: String,
        parameters: HashMap<String, String>,
    ) -> Result<ReferenceValue>;
}

/// The struct `Extractors` is responsible for implementing
/// trait `Extrators`.
/// `extractors_module_list` is a map that maps provenance type
/// (in String) to its Extractor's instancializer.
/// `extractors_instance_map` is another map that maps provenance type
/// to the instancialized Extractor. The two map implement a 
/// "instance-on-demand" mechanism.
pub struct Extrators {
    extractors_module_list: ExtractorModuleList,
    extractors_instance_map: HashMap<String, ExtractorInstance>,
}

impl Default for Extrators {
    fn default() -> Self {
        Self::new()
    }
}

impl Extrators {
    pub fn new() -> Self {
        let extractors_module_list = ExtractorModuleList::new();
        let extractors_instance_map = HashMap::new();
        Extrators {
            extractors_module_list,
            extractors_instance_map,
        }
    }

    /// Register an `Extractor` instance to `Extractors`. The `Extractor` is responsible for 
    /// handling specific kind of provenance (as `extractor_name` indicates).
    fn register_instance(&mut self, extractor_name: String, extractor_instance: ExtractorInstance) {
        self.extractors_instance_map.insert(extractor_name, extractor_instance);
    }

    /// Instantiate an `Extractor` of given type `extractor_name`. This method will
    /// instantiate an `Extractor` instance and then register it.
    fn instantiate_extractor(&mut self, extractor_name: String) -> Result<()> {
        let instantiate_func = self.extractors_module_list.get_func(&extractor_name)?;
        let extractor_instance = (instantiate_func)();
        self.register_instance(extractor_name, extractor_instance);
        Ok(())
    }
}

impl ExtratorsAPI for Extrators {
    /// Process the input provenance, verify its authenticity, and
    /// output the reference value if the verification succeeds.
    /// * `provenance_type` is the type of the provenance, e.g. "in-toto".
    /// * `provenance_name` is the artifact's name, e.g. "kata-agent".
    /// * `provenance` is the content of provenance (if needed).
    /// * `parameters` is the parameters for Extractor to handle the 
    /// given provenance.
    fn handle_provenance(
        &mut self,
        provenance_type: String,
        provenance_name: String,
        provenance: String,
        parameters: HashMap<String, String>,
    ) -> Result<ReferenceValue> {
        if self.extractors_instance_map.get_mut(&provenance_type).is_none() {
            self.instantiate_extractor(provenance_type.clone())?;
        }
        let extractor_instance = self
            .extractors_instance_map
            .get_mut(&provenance_type)
            .ok_or_else(|| anyhow!("The Extractor instance does not existing!"))?;

        // Before verify and extract, the process should change current dir 
        // to the parameters['working_dir'], usually a temp dir.
        // After verify_and_extract(), reset the process's current dir
        let cwd = env::current_dir()?
            .as_path()
            .to_owned();
        
        let working_dir = parameters
            .get(WORKING_DIR_KEY)
            .ok_or(anyhow!("parameters do not indicate {}!", WORKING_DIR_KEY))?
            .to_owned();

        env::set_current_dir(Path::new(&working_dir))?;

        let hash_value = extractor_instance.verify_and_extract(
            provenance,
            parameters
        )?;

        // Reset the current directory
        env::set_current_dir(cwd)?;

        Ok(ReferenceValue {
            hash_value,
            provenance_name,
        })
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{Extrators, ExtratorsAPI, WORKING_DIR_KEY};

    #[test]
    fn in_toto() {
        let mut extractors = Extrators::new();
        let mut parameters = HashMap::new();

        // the following operations should be done by the Wares in PreProcessor
        parameters.insert(WORKING_DIR_KEY.to_string(), "../tests/in-toto".to_string());
        parameters.insert("layout_path".to_string(), "demo.layout".to_string());

        let pub_key_paths = {
            let pub_key_path: String = "./alice.pub".to_string();
            let pub_key_paths = vec![pub_key_path];
            serde_json::to_string(&pub_key_paths).unwrap()
        };

        parameters.insert("pub_key_paths".to_string(), pub_key_paths);

        let intermediate_paths = {
            let intermediate_paths: Vec<String> = vec![];
            serde_json::to_string(&intermediate_paths).unwrap()
        };

        parameters.insert("intermediate_paths".to_string(), intermediate_paths);
        parameters.insert("link_dir".to_string(), ".".to_string());
        parameters.insert("line_normalization".to_string(), "true".to_string());
        
        match extractors.handle_provenance(
            "in-toto".to_string(), 
            "foo.tar.gz".to_string(), 
            "".to_string(), 
            parameters,
        ) {
            Ok(_) => panic!("test failed!"),
            Err(e) => {
                // Now in-toto is now fully developed
                assert_eq!(e.to_string(), "Can not extract hash value using in-toto");
            },
        };
    }
}