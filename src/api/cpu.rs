use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Cpu {
  X86,
  X86_64,
  Ppc,
  Ppc64,
  Sparc,
  Arm,
  Arm64,
  Mips,
  Mips64,
  Unknown,
}

impl From<minidump::system_info::Cpu> for Cpu {
  fn from(cpu: minidump::system_info::Cpu) -> Self {
    match cpu {
      minidump::system_info::Cpu::X86 => Self::X86,
      minidump::system_info::Cpu::X86_64 => Self::X86_64,
      minidump::system_info::Cpu::Arm => Self::Arm,
      minidump::system_info::Cpu::Arm64 => Self::Arm64,
      minidump::system_info::Cpu::Ppc => Self::Ppc,
      minidump::system_info::Cpu::Ppc64 => Self::Ppc64,
      minidump::system_info::Cpu::Sparc => Self::Sparc,
      minidump::system_info::Cpu::Mips => Self::Mips,
      minidump::system_info::Cpu::Mips64 => Self::Mips64,
      _ => Self::Unknown,
    }
  }
}
