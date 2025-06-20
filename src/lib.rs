mod careersavegame;
mod economy;
mod stats;

pub use {
  careersavegame::*,
  economy::*,
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
    let ext = match self {
      Self::Json => "json",
      Self::Xml => "xml"
    };
    write!(f, "{ext}")
  }
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
    let file = match self {
      Self::CareerSavegame => "careerSavegame",
      Self::Vehicles => "vehicles",
      Self::Economy => "economy"
    };
    write!(f, "{file}")
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
      base_url: Cow::Owned(format!("http://{ip}/feed")),
      code,
      format
    }
  }

  /// Returns the string containing the `/feed/dedicated-server-stats` endpoint
  pub fn stats(&self) -> String { format!("{}/dedicated-server-stats.{}?code={}", self.base_url, self.format, self.code) }

  /// Returns the string containing the `/feed/dedicated-server-savegame` endpoint with chosen filename
  pub fn savegame(
    &self,
    filename: Filename
  ) -> String {
    format!("{}/dedicated-server-savegame.html?code={}&file={filename}", self.base_url, self.code)
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
  pub author:      Option<String>,
  pub hash:        Option<String>,
  /// Filename, e.g "FS25_precisionFarming"
  pub name:        Option<String>,
  pub version:     Option<String>,
  /// Friendly name, e.g "Precision Farming"
  pub description: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsgMod {
  /// Filename, e.g "FS25_precisionFarming"
  #[serde(rename = "@modName")]
  pub mod_name:  Option<String>,
  /// Friendly name, e.g "Precision Farming"
  #[serde(rename = "@title")]
  pub title:     Option<String>,
  #[serde(rename = "@version")]
  pub version:   Option<String>,
  #[serde(rename = "@required")]
  pub required:  Option<String>,
  #[serde(rename = "@fileHash")]
  pub file_hash: Option<String>
}

impl Mod {
  /// Retrieve the mod's name aka filename
  pub fn name(&self) -> Option<&str> {
    match self {
      Self::DssFormat(dss_mod) => dss_mod.name.as_deref(),
      Self::CsgFormat(csg_mod) => csg_mod.mod_name.as_deref()
    }
  }

  /// Retrieve the mod's version
  pub fn version(&self) -> Option<&str> {
    match self {
      Self::DssFormat(dss_mod) => dss_mod.version.as_deref(),
      Self::CsgFormat(csg_mod) => csg_mod.version.as_deref()
    }
  }

  /// Retrieve the mod's MD5 hash
  pub fn hash(&self) -> Option<&str> {
    match self {
      Self::DssFormat(dss_mod) => dss_mod.hash.as_deref(),
      Self::CsgFormat(csg_mod) => csg_mod.file_hash.as_deref()
    }
  }

  /// Retrieve the mod's description aka friendly name
  pub fn description(&self) -> Option<&str> {
    match self {
      Self::DssFormat(dss_mod) => dss_mod.description.as_deref(),
      Self::CsgFormat(csg_mod) => csg_mod.title.as_deref()
    }
  }
}
