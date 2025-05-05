// SPDX-License-Identifier: Apache-2.0

use crate::{WifiError, WifiIface};

use zbus::zvariant;

const DBUS_INTERFACE_ROOT: &str = "fi.w1.wpa_supplicant1";

// These proxy() macros only generate private struct, hence it should be
// sit with its consumer.
#[zbus::proxy(
    interface = "fi.w1.wpa_supplicant1",
    default_service = "fi.w1.wpa_supplicant1",
    default_path = "/fi/w1/wpa_supplicant1"
)]
trait WpaDbus {
    #[zbus(property)]
    fn interfaces(&self) -> zbus::Result<Vec<zvariant::OwnedObjectPath>>;
}

#[derive(Clone, Debug)]
pub(crate) struct WpaDbusConnection<'a> {
    pub(crate) connection: zbus::Connection,
    proxy: WpaDbusProxy<'a>,
}

impl WpaDbusConnection<'_> {
    pub(crate) const IFACE_ROOT: &'static str = DBUS_INTERFACE_ROOT;

    pub(crate) async fn new() -> Result<Self, WifiError> {
        let connection = zbus::Connection::system().await?;
        let proxy = WpaDbusProxy::new(&connection).await?;
        Ok(Self { connection, proxy })
    }

    pub(crate) async fn get_ifaces(&self) -> Result<Vec<WifiIface>, WifiError> {
        let obj_paths: Vec<String> = self
            .proxy
            .interfaces()
            .await?
            .into_iter()
            .map(obj_path_to_string)
            .collect();
        let mut ret = Vec::new();
        for obj_path in obj_paths {
            ret.push(self.get_iface(obj_path.as_str()).await?);
        }

        Ok(ret)
    }
}

pub(crate) fn obj_path_to_string(
    obj_path: zvariant::OwnedObjectPath,
) -> String {
    obj_path.into_inner().to_string()
}
