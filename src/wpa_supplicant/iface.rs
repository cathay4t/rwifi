// SPDX-License-Identifier: Apache-2.0

use crate::WifiError;

use super::dbus::WpaDbusConnection;

const DBUS_INTERFACE_IFACE: &str = "fi.w1.wpa_supplicant1.Interface";

impl WpaDbusConnection<'_> {
    pub(crate) async fn get_ifname(
        &self,
        obj_path: &str,
    ) -> Result<String, WifiError> {
        let proxy = zbus::Proxy::new(
            &self.connection,
            Self::IFACE_ROOT,
            obj_path,
            DBUS_INTERFACE_IFACE,
        )
        .await?;
        Ok(proxy.get_property::<String>("Ifname").await?)
    }
}
