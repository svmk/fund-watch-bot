use crate::prelude::*;
use crate::serializer::serializer::Serializer;
use crate::serializer::service::yaml_serializer::YamlSerializer;
use crate::system::app_config::AppConfig;
use std::path::Path;
use std::fs::File;

pub fn app_config_from_path(path: &Path) -> Result<AppConfig, Failure> {
    let mut file = File::open(path)?;
    let serializer = YamlSerializer::new();
    let config = serializer.from_reader(&mut file)?;
    return Ok(config);
}