use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Os {
  Windows,
  MacOs,
  Ios,
  Linux,
  Solaris,
  Android,
  Ps3,
  NaCl,
  Unknown,
}

impl From<minidump::system_info::Os> for Os {
  fn from(os: minidump::system_info::Os) -> Self {
    match os {
      minidump::system_info::Os::Windows => Self::Windows,
      minidump::system_info::Os::MacOs => Self::MacOs,
      minidump::system_info::Os::Ios => Self::Ios,
      minidump::system_info::Os::Linux => Self::Linux,
      minidump::system_info::Os::Solaris => Self::Solaris,
      minidump::system_info::Os::Android => Self::Android,
      minidump::system_info::Os::Ps3 => Self::Ps3,
      minidump::system_info::Os::NaCl => Self::NaCl,
      minidump::system_info::Os::Unknown(_) => Self::Unknown,
    }
  }
}
