use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrashReason {
  /// A Mac/iOS error code with no other interesting details.
  MacGeneral,
  MacBadAccessKern,
  MacBadAccessArm,
  MacBadAccessPpc,
  MacBadAccessX86,
  MacBadInstructionArm,
  MacBadInstructionPpc,
  MacBadInstructionX86,
  MacArithmeticArm,
  MacArithmeticPpc,
  MacArithmeticX86,
  MacSoftware,
  MacBreakpointArm,
  MacBreakpointPpc,
  MacBreakpointX86,
  MacResource,
  MacGuard,

  /// A Linux/Android error code with no other interesting metadata.
  LinuxGeneral,
  LinuxSigill,
  LinuxSigtrap,
  LinuxSigbus,
  LinuxSigfpe,
  LinuxSigsegv,
  LinuxSigsys,

  /// A Windows error code with no other interesting metadata.
  WindowsGeneral,
  /// A Windows error from winerror.h.
  WindowsWinError,
  /// A Windows error for a specific facility from winerror.h.
  WindowsWinErrorWithFacility,
  /// A Windows error from ntstatus.h
  WindowsNtStatus,
  /// ExceptionCodeWindows::EXCEPTION_ACCESS_VIOLATION but with details on the kind of access.
  WindowsAccessViolation,
  /// ExceptionCodeWindows::EXCEPTION_IN_PAGE_ERROR but with details on the kind of access.
  /// Second argument is a windows NTSTATUS value.
  WindowsInPageError,
  /// ExceptionCodeWindows::EXCEPTION_STACK_BUFFER_OVERRUN with an accompanying
  /// windows FAST_FAIL value.
  WindowsStackBufferOverrun,
  /// A Windows error with no known mapping.
  WindowsUnknown,

  Unknown,
}

impl From<minidump::CrashReason> for CrashReason {
  fn from(value: minidump::CrashReason) -> Self {
    match value {
      minidump::CrashReason::MacGeneral(_, _) => Self::MacGeneral,
      minidump::CrashReason::MacBadAccessKern(_) => Self::MacBadAccessKern,
      minidump::CrashReason::MacBadAccessArm(_) => Self::MacBadAccessArm,
      minidump::CrashReason::MacBadAccessPpc(_) => Self::MacBadAccessPpc,
      minidump::CrashReason::MacBadAccessX86(_) => Self::MacBadAccessX86,
      minidump::CrashReason::MacBadInstructionArm(_) => Self::MacBadInstructionArm,
      minidump::CrashReason::MacBadInstructionPpc(_) => Self::MacBadInstructionPpc,
      minidump::CrashReason::MacBadInstructionX86(_) => Self::MacBadInstructionX86,
      minidump::CrashReason::MacArithmeticArm(_) => Self::MacArithmeticArm,
      minidump::CrashReason::MacArithmeticPpc(_) => Self::MacArithmeticPpc,
      minidump::CrashReason::MacArithmeticX86(_) => Self::MacArithmeticX86,
      minidump::CrashReason::MacSoftware(_) => Self::MacSoftware,
      minidump::CrashReason::MacBreakpointArm(_) => Self::MacBreakpointArm,
      minidump::CrashReason::MacBreakpointPpc(_) => Self::MacBreakpointPpc,
      minidump::CrashReason::MacBreakpointX86(_) => Self::MacBreakpointX86,
      minidump::CrashReason::MacResource(_, _, _) => Self::MacResource,
      minidump::CrashReason::MacGuard(_, _, _) => Self::MacGuard,
      minidump::CrashReason::LinuxGeneral(_, _) => Self::LinuxGeneral,
      minidump::CrashReason::LinuxSigill(_) => Self::LinuxSigill,
      minidump::CrashReason::LinuxSigtrap(_) => Self::LinuxSigtrap,
      minidump::CrashReason::LinuxSigbus(_) => Self::LinuxSigbus,
      minidump::CrashReason::LinuxSigfpe(_) => Self::LinuxSigfpe,
      minidump::CrashReason::LinuxSigsegv(_) => Self::LinuxSigsegv,
      minidump::CrashReason::LinuxSigsys(_) => Self::LinuxSigsys,
      minidump::CrashReason::WindowsGeneral(_) => Self::WindowsGeneral,
      minidump::CrashReason::WindowsWinError(_) => Self::WindowsWinError,
      minidump::CrashReason::WindowsWinErrorWithFacility(_, _) => Self::WindowsWinErrorWithFacility,
      minidump::CrashReason::WindowsNtStatus(_) => Self::WindowsNtStatus,
      minidump::CrashReason::WindowsAccessViolation(_) => Self::WindowsAccessViolation,
      minidump::CrashReason::WindowsInPageError(_, _) => Self::WindowsInPageError,
      minidump::CrashReason::WindowsStackBufferOverrun(_) => Self::WindowsStackBufferOverrun,
      minidump::CrashReason::WindowsUnknown(_) => Self::WindowsUnknown,
      minidump::CrashReason::Unknown(_, _) => Self::Unknown,
    }
  }
}

impl std::fmt::Display for CrashReason {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CrashReason::MacGeneral => write!(f, "Mac General Error"),
      CrashReason::MacBadAccessKern => write!(f, "Mac Bad Access (Kernel)"),
      CrashReason::MacBadAccessArm => write!(f, "Mac Bad Access (ARM)"),
      CrashReason::MacBadAccessPpc => write!(f, "Mac Bad Access (PPC)"),
      CrashReason::MacBadAccessX86 => write!(f, "Mac Bad Access (x86)"),
      CrashReason::MacBadInstructionArm => write!(f, "Mac Bad Instruction (ARM)"),
      CrashReason::MacBadInstructionPpc => write!(f, "Mac Bad Instruction (PPC)"),
      CrashReason::MacBadInstructionX86 => write!(f, "Mac Bad Instruction (x86)"),
      CrashReason::MacArithmeticArm => write!(f, "Mac Arithmetic Error (ARM)"),
      CrashReason::MacArithmeticPpc => write!(f, "Mac Arithmetic Error (PPC)"),
      CrashReason::MacArithmeticX86 => write!(f, "Mac Arithmetic Error (x86)"),
      CrashReason::MacSoftware => write!(f, "Mac Software Error"),
      CrashReason::MacBreakpointArm => write!(f, "Mac Breakpoint (ARM)"),
      CrashReason::MacBreakpointPpc => write!(f, "Mac Breakpoint (PPC)"),
      CrashReason::MacBreakpointX86 => write!(f, "Mac Breakpoint (x86)"),
      CrashReason::MacResource => write!(f, "Mac Resource Error"),
      CrashReason::MacGuard => write!(f, "Mac Guard Error"),
      CrashReason::LinuxGeneral => write!(f, "Linux General Error"),
      CrashReason::LinuxSigill => write!(f, "SIGILL"),
      CrashReason::LinuxSigtrap => write!(f, "SIGTRAP"),
      CrashReason::LinuxSigbus => write!(f, "SIGBUS"),
      CrashReason::LinuxSigfpe => write!(f, "SIGFPE"),
      CrashReason::LinuxSigsegv => write!(f, "SIGSEGV"),
      CrashReason::LinuxSigsys => write!(f, "SIGSYS"),
      CrashReason::WindowsGeneral => write!(f, "Windows General Error"),
      CrashReason::WindowsWinError => write!(f, "Windows WinError"),
      CrashReason::WindowsWinErrorWithFacility => write!(f, "Windows WinError with Facility"),
      CrashReason::WindowsNtStatus => write!(f, "Windows NT Status"),
      CrashReason::WindowsAccessViolation => write!(f, "Windows Access Violation"),
      CrashReason::WindowsInPageError => write!(f, "Windows In Page Error"),
      CrashReason::WindowsStackBufferOverrun => write!(f, "Windows Stack Buffer Overrun"),
      CrashReason::WindowsUnknown => write!(f, "Windows Unknown Error"),
      CrashReason::Unknown => write!(f, "Unknown Error"),
    }
  }
}
