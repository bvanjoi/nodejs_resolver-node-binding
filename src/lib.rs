use napi::bindgen_prelude::External;
use napi_derive::napi;
use nodejs_resolver::{AliasMap, Resolver, ResolverOptions};
use serde::Deserialize;
use std::{
  path::{Path, PathBuf},
};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[napi(object)]
pub struct Alias {
  pub key: String,
  pub value: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[napi(object)]
pub struct RawResolverOptions {
  pub extensions: Option<Vec<String>>,
  pub enforce_extension: Option<bool>,
  pub alias: Option<Vec<Alias>>,
  pub alias_fields: Option<Vec<String>>,
  pub condition_names: Option<Vec<String>>,
  pub symlinks: Option<bool>,
  pub description_file: Option<Option<String>>,
  pub main_files: Option<Vec<String>>,
  pub main_fields: Option<Vec<String>>,
  pub modules: Option<Vec<String>>,
  pub prefer_relative: Option<bool>,
  pub enable_unsafe_cache: Option<bool>,
  pub tsconfig_path: Option<String>,
}

impl RawResolverOptions {
  pub fn normalized(&self) -> ResolverOptions {
    let default = ResolverOptions::default();
    ResolverOptions {
      enforce_extension: self.enforce_extension.to_owned(),
      extensions: self.extensions.to_owned().unwrap_or(default.extensions),
      alias: self.alias.to_owned().map_or(default.alias, parse_alias),
      alias_fields: self.alias_fields.to_owned().unwrap_or(default.alias_fields),
      condition_names: self
        .condition_names
        .to_owned()
        .map_or(default.condition_names, |vec| vec.into_iter().collect()),
      symlinks: self.symlinks.unwrap_or(default.symlinks),
      description_file: self
      .description_file
      .to_owned()
      .unwrap_or(default.description_file),
      main_files: self.main_files.to_owned().unwrap_or(default.main_files),
      main_fields: self.main_fields.to_owned().unwrap_or(default.main_fields),
      prefer_relative: self.prefer_relative.unwrap_or(default.prefer_relative),
      enable_unsafe_cache: self
        .enable_unsafe_cache
        .to_owned()
        .unwrap_or(default.enable_unsafe_cache),
      tsconfig: self.tsconfig_path.to_owned().map(PathBuf::from),
    }
  }
}

fn parse_alias(alias: Vec<Alias>) -> Vec<(String, AliasMap)> {
  alias
    .into_iter()
    .map(|item| {
      (
        item.key,
        item.value.map_or(AliasMap::Ignored, AliasMap::Target),
      )
    })
    .collect()
}

#[napi(object)]
pub struct ResolverInternal {}

#[napi(ts_return_type = "ExternalObject<ResolverInternal>")]
pub fn create(options: RawResolverOptions) -> Result<External<Resolver>, napi::Error> {
  let options = options.normalized();
  let resolver = Resolver::new(options);
  Ok(External::new(resolver))
}

#[napi(object)]
pub struct ResolveResult {
  pub status: bool,
  pub path: Option<String>,
}

#[napi(
  ts_args_type = "resolver: ExternalObject<ResolverInternal>, base_dir: string, id: string",
  ts_return_type = "{status: boolean, path?: string}"
)]
pub fn resolve(
  resolver: External<Resolver>,
  base_dir: String,
  id: String,
) -> Result<ResolveResult, napi::Error> {
  match (*resolver).resolve(Path::new(&base_dir), &id) {
    Ok(val) => {
      if let nodejs_resolver::ResolverResult::Info(info) = val {
        Ok(ResolveResult {
          status: true,
          path: Some(format!("{}{}{}", info.path.display(), &info.request.query, &info.request.fragment)),
        })
      } else {
        Ok(ResolveResult {
          status: false,
          path: None,
        })
      }
    }
    Err(err) => Err(napi::Error::new(napi::Status::GenericFailure, err)),
  }
}
