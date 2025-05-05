// SPDX-License-Identifier: Apache-2.0

use crate::WifiError;

use super::dbus::WpaDbusConnection;

const DBUS_INTERFACE_BSS: &str = "fi.w1.wpa_supplicant1.BSS";

impl WpaDbusConnection<'_> {
    pub(crate) async fn get_bss_ssid(
        &self,
        obj_path: &str,
    ) -> Result<String, WifiError> {
        let proxy = zbus::Proxy::new(
            &self.connection,
            Self::IFACE_ROOT,
            obj_path,
            DBUS_INTERFACE_BSS,
        )
        .await?;
        let ssid_u8 = proxy.get_property::<Vec<u8>>("SSID").await?;
        Ok(std::str::from_utf8(&ssid_u8)?.to_string())
    }

    pub(crate) async fn get_bss_bssid(
        &self,
        obj_path: &str,
    ) -> Result<[u8; 6], WifiError> {
        let proxy = zbus::Proxy::new(
            &self.connection,
            Self::IFACE_ROOT,
            obj_path,
            DBUS_INTERFACE_BSS,
        )
        .await?;
        let bssid_vec = proxy.get_property::<Vec<u8>>("BSSID").await?;
        let mut raw = [0u8; 6];
        if bssid_vec.len() >= 6 {
            raw.copy_from_slice(&bssid_vec[..6])
        } else {
            raw.copy_from_slice(&bssid_vec[..bssid_vec.len()])
        }
        Ok(raw)
    }
}
