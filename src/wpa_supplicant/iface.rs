// SPDX-License-Identifier: Apache-2.0

use crate::{WifiError, WifiIface};

use zbus::zvariant;

use super::dbus::{WpaDbusConnection, obj_path_to_string};

const DBUS_INTERFACE_IFACE: &str = "fi.w1.wpa_supplicant1.Interface";

impl WpaDbusConnection<'_> {
    pub(crate) async fn get_iface(
        &self,
        obj_path: &str,
    ) -> Result<WifiIface, WifiError> {
        let bss_obj_path = self.get_iface_cur_bss(obj_path).await?;
        let ssid = if let Some(bss_obj_path) = &bss_obj_path {
            Some(self.get_bss_ssid(bss_obj_path).await?)
        } else {
            None
        };
        let bssid = if let Some(bss_obj_path) = &bss_obj_path {
            Some(self.get_bss_bssid(bss_obj_path).await?)
        } else {
            None
        };

        Ok(WifiIface {
            name: self.get_iface_ifname(obj_path).await?,
            ssid,
            bssid,
        })
    }

    async fn get_iface_ifname(
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

    async fn get_iface_cur_bss(
        &self,
        obj_path: &str,
    ) -> Result<Option<String>, WifiError> {
        let proxy = zbus::Proxy::new(
            &self.connection,
            Self::IFACE_ROOT,
            obj_path,
            DBUS_INTERFACE_IFACE,
        )
        .await?;
        let current_bss = proxy
            .get_property::<zvariant::OwnedObjectPath>("CurrentBSS")
            .await
            .map(obj_path_to_string)?;

        if current_bss == "/" {
            Ok(None)
        } else {
            Ok(Some(current_bss))
        }
    }
}
