// SPDX-License-Identifier: Apache-2.0

use crate::{WifiError, WifiIface};

use super::dbus::WpaDbusConnection;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct WpaSupplicantConnection<'a> {
    pub(crate) dbus: WpaDbusConnection<'a>,
}

impl WpaSupplicantConnection<'_> {
    pub async fn new() -> Result<Self, WifiError> {
        Ok(Self {
            dbus: WpaDbusConnection::new().await?,
        })
    }

    pub async fn get_ifaces(&self) -> Result<Vec<WifiIface>, WifiError> {
        self.dbus.get_ifaces().await
    }
}
