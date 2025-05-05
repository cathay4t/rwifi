// SPDX-License-Identifier: Apache-2.0

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    #[default]
    Bug,
    DbusError,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WifiError {
    pub kind: ErrorKind,
    pub msg: String,
}

impl std::fmt::Display for WifiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.msg)
    }
}

impl WifiError {
    pub fn new(kind: ErrorKind, msg: String) -> Self {
        Self { kind, msg }
    }
}

impl std::error::Error for WifiError {}

impl From<zbus::Error> for WifiError {
    fn from(e: zbus::Error) -> Self {
        Self {
            kind: ErrorKind::DbusError,
            msg: e.to_string(),
        }
    }
}
