// SPDX-License-Identifier: Apache-2.0

use rwifi::WpaSupplicantConnection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = WpaSupplicantConnection::new().await?;

    for iface in conn.get_ifaces().await? {
        println!("HAHA {iface:?}");
    }
    Ok(())
}
