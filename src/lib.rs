use napi::bindgen_prelude::External;
use napi_derive::napi;
use nodejs_resolver::{AliasMap, Resolver, ResolverOptions, ResolverCache, SideEffects};
use serde::Deserialize;
use std::{
  path::{Path, PathBuf},
  sync::Arc,
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
  pub browser_field: Option<bool>,
  pub condition_names: Option<Vec<String>>,
  pub symlinks: Option<bool>,
  pub description_file: Option<Option<String>>,
  pub main_files: Option<Vec<String>>,
  pub main_fields: Option<Vec<String>>,
  pub modules: Option<Vec<String>>,
  pub prefer_relative: Option<bool>,
  pub tsconfig_path: Option<String>,
}

impl RawResolverOptions {
  pub fn normalized(&self, external_cache: Option<Arc<ResolverCache>>) -> ResolverOptions {
    let default = ResolverOptions::default();
    ResolverOptions {
      enforce_extension: self.enforce_extension.to_owned(),
      extensions: self.extensions.to_owned().unwrap_or(default.extensions),
      alias: self.alias.to_owned().map_or(default.alias, parse_alias),
      browser_field: self
        .browser_field
        .to_owned()
        .unwrap_or(default.browser_field),
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
      tsconfig: self.tsconfig_path.to_owned().map(PathBuf::from),
      external_cache,
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
  let options = options.normalized(None);
  let resolver = Resolver::new(options);
  Ok(External::new(resolver))
}

#[napi(object)]
pub struct ResolverCacheInternal {}

#[napi(ts_return_type = "ExternalObject<ResolverCacheInternal>")]
pub fn create_external_cache() -> Result<External<Arc<ResolverCache>>, napi::Error> {
  Ok(External::new(
    Arc::new(ResolverCache::default()),
  ))
}

#[napi(
  ts_args_type = "options: RawResolverOptions, external_cache: ExternalObject<ResolverCacheInternal>",
  ts_return_type = "ExternalObject<ResolverInternal>"
)]
pub fn create_with_external_cache(
  options: RawResolverOptions,
  external_cache: External<Arc<ResolverCache>>,
) -> Result<External<Resolver>, napi::Error> {
  let external_cache = external_cache.as_ref().clone();
  let options = options.normalized(Some(external_cache));
  let resolver = Resolver::new(options);
  Ok(External::new(resolver))
}

#[napi(
  ts_args_type = "resolver: ExternalObject<ResolverInternal>, base_dir: string, id: string",
  ts_return_type = "string"
)]
pub fn resolve(
  resolver: External<Resolver>,
  base_dir: String,
  id: String,
) -> Result<String, napi::Error> {
  match (*resolver).resolve(Path::new(&base_dir), &id) {
    Ok(val) => {
      if let nodejs_resolver::ResolverResult::Info(info) = val {
        Ok(format!(
          "{}{}{}",
          info.path.display(),
          &info.request.query,
          &info.request.fragment
        ))
      } else {
        Ok(String::from("false"))
      }
    }
    Err(err) => Err(napi::Error::new(napi::Status::GenericFailure, err)),
  }
}

#[napi(object)]
pub struct SideEffectsStats {
  pub bool_val: Option<bool>,
  pub array_val: Option<Vec<String>>,
  pub pkg_file_path: String,
}

#[napi(
  ts_args_type = "resolver: ExternalObject<ResolverInternal>, path: string",
  ts_return_type = "{boolVal?: boolean, arrayVal?: string[], pkgFilePath: string} | undefined"
)]
pub fn load_side_effects(
  resolver: External<Resolver>,
  path: String,
) -> Result<Option<SideEffectsStats>, napi::Error> {
  match (*resolver).load_side_effects(&Path::new(&path)) {
    Ok(val) => Ok(val.map(|val| {
      let (bool_val, array_val) = val
        .1
        .map(|side_effects| match side_effects {
          SideEffects::Bool(bool) => (Some(bool), None),
          SideEffects::Array(array) => (None, Some(array)),
        })
        .unwrap_or((None, None));
      SideEffectsStats {
        pkg_file_path: val.0.display().to_string(),
        bool_val,
        array_val,
      }
    })),
    Err(err) => Err(napi::Error::new(napi::Status::GenericFailure, err)),
  }
}
