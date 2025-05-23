// SPDX-License-Identifier: Apache-2.0

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct WifiIface {
    pub name: String,
    pub ssid: Option<String>,
    pub bssid: Option<[u8; 6]>,
}
