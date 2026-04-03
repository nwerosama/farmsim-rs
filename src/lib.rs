mod careersavegame;
mod economy;
mod environment;
mod farms;
mod stats;
pub mod utils;

pub use {
  careersavegame::*,
  economy::*,
  environment::*,
  farms::*,
  stats::*
};

use {
  serde::{
    Deserialize,
    Serialize
  },
  std::{
    borrow::Cow,
    fmt
  }
};

/// This is only effective on DSS field in [Endpoint] struct
#[derive(Debug, Clone, Copy)]
pub enum Format {
  Json,
  Xml
}

impl fmt::Display for Format {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>
  ) -> fmt::Result {
    match self {
      Self::Json => "json",
      Self::Xml => "xml"
    }
    .fmt(f)
  }
}

/// Validate the data before doing something with it<br>
/// ***Note:*** You must add your own validation code to use this!
pub trait Validation {
  fn is_valid(&self) -> bool;
}

/// List of supported filenames in the `dedicated-server-savegame` endpoint
#[derive(Debug, Clone, Copy)]
pub enum Filename {
  CareerSavegame,
  Vehicles,
  Economy
}

impl fmt::Display for Filename {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>
  ) -> fmt::Result {
    match self {
      Self::CareerSavegame => "careerSavegame",
      Self::Vehicles => "vehicles",
      Self::Economy => "economy"
    }
    .fmt(f)
  }
}

pub struct EndpointBuilder {
  /// Either `example.com` or `1.2.3.4:8080`
  ip:     Cow<'static, str>,
  /// This is your API code found on the webinterface's settings page<br>
  /// Something like `?code=<your API code>` in the URL(s)
  code:   Cow<'static, str>,
  /// Only effective on DSS field in [Endpoint] struct
  format: Format
}

impl EndpointBuilder {
  pub fn new(
    ip: &str,
    code: &str
  ) -> Self {
    // todo; use proper pattern matching to cover possible edge cases
    let ip = ip.strip_prefix("http://").unwrap_or(ip);

    Self {
      ip:     Cow::Owned(ip.into()),
      code:   Cow::Owned(code.into()),
      format: Format::Json
    }
  }

  /// Sets the data format type for dedicated-server-stats (DSS) endpoint
  pub fn format(
    mut self,
    format: Format
  ) -> Self {
    self.format = format;
    self
  }

  pub fn build(self) -> Endpoint { Endpoint::new(self.ip, self.code, self.format) }
}

pub struct Endpoint {
  base_url: Cow<'static, str>,
  code:     Cow<'static, str>,
  format:   Format
}

impl Endpoint {
  fn new(
    ip: Cow<'static, str>,
    code: Cow<'static, str>,
    format: Format
  ) -> Self {
    Self {
      base_url: Cow::Owned(format!("http://{ip}")),
      code,
      format
    }
  }

  /// Returns the string containing the `/feed/dedicated-server-stats` endpoint
  pub fn stats(&self) -> String { format!("{}/feed/dedicated-server-stats.{}?code={}", self.base_url, self.format, self.code) }

  /// Returns the string containing the `/feed/dedicated-server-savegame`
  /// endpoint with chosen filename
  pub fn savegame(
    &self,
    filename: Filename
  ) -> String {
    format!("{}/feed/dedicated-server-savegame.html?code={}&file={filename}", self.base_url, self.code)
  }

  /// Returns the string containing the mods endpoint, direct download is
  /// provided if `all_mods` is enabled else links to panel's mods tab instead
  pub fn mods(
    &self,
    all_mods: bool
  ) -> String {
    match all_mods {
      true => format!("{}/all_mods_download?onlyActive=true", self.base_url),
      false => format!("{}/mods.html", self.base_url)
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Mod {
  /// JSON format from dedicated-server-stats data
  DssFormat(DssMod),
  /// XML format from dedicated-server-savegame data
  CsgFormat(CsgMod)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DssMod {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub author:      Option<Box<str>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub hash:        Option<Box<str>>,
  /// Filename, e.g "FS25_precisionFarming"
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name:        Option<Box<str>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version:     Option<Box<str>>,
  /// Friendly name, e.g "Precision Farming"
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<Box<str>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsgMod {
  /// Filename, e.g "FS25_precisionFarming"
  #[serde(rename = "@modName", skip_serializing_if = "Option::is_none")]
  pub mod_name:  Option<Box<str>>,
  /// Friendly name, e.g "Precision Farming"
  #[serde(rename = "@title", skip_serializing_if = "Option::is_none")]
  pub title:     Option<Box<str>>,
  #[serde(rename = "@version", skip_serializing_if = "Option::is_none")]
  pub version:   Option<Box<str>>,
  #[serde(rename = "@required", skip_serializing_if = "Option::is_none")]
  pub required:  Option<Box<str>>,
  #[serde(rename = "@fileHash", skip_serializing_if = "Option::is_none")]
  pub file_hash: Option<Box<str>>
}

impl Mod {
  /// Retrieve the mod's `name` aka filename
  pub fn name(&self) -> Option<&str> {
    match self {
      Self::DssFormat(dss_mod) => dss_mod.name.as_deref(),
      Self::CsgFormat(csg_mod) => csg_mod.mod_name.as_deref()
    }
  }

  /// Retrieve the mod's `version`
  pub fn version(&self) -> Option<&str> {
    match self {
      Self::DssFormat(dss_mod) => dss_mod.version.as_deref(),
      Self::CsgFormat(csg_mod) => csg_mod.version.as_deref()
    }
  }

  /// Retrieve the mod's `MD5 hash`
  pub fn hash(&self) -> Option<&str> {
    match self {
      Self::DssFormat(dss_mod) => dss_mod.hash.as_deref(),
      Self::CsgFormat(csg_mod) => csg_mod.file_hash.as_deref()
    }
  }

  /// Retrieve the mod's `description` aka friendly name
  pub fn description(&self) -> Option<&str> {
    match self {
      Self::DssFormat(dss_mod) => dss_mod.description.as_deref(),
      Self::CsgFormat(csg_mod) => csg_mod.title.as_deref()
    }
  }

  /// Retrieve the mod's `required`<br>
  /// **Note: This is only retrievable from CSG property! Accessing this from
  /// DSS will result in a panic**
  pub fn required(&self) -> Option<&str> {
    match self {
      Self::DssFormat(_) => panic!("Access this from CSG instead as DSS does not offer it"),
      Self::CsgFormat(csg_mod) => csg_mod.required.as_deref()
    }
  }
}
