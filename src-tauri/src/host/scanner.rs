// This module is used to send the heartbeat to the dev host to indicate that the debugger is running.

use std::collections::HashMap;

use debug_print::debug_println;

use crate::constant::{DEFAULT_HOSTNAME, DEFAULT_PATH};

pub async fn heartbeat(url: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let url = format!("{}{}/heartbeat", url, DEFAULT_PATH);
    // TODO: send heartbeat to the dev host
    let resp = reqwest::get(url).await?.text().await?;
    // debug_println!("heartbeat resp: {}", resp);
    // log::info!("heartbeat resp: {}", resp);
    if resp.len() > 0 {
        if resp.contains("costa_heartbeat_success_") {
            // heartbeat success, resp is host device info
            return Ok(Some(
                resp.split("costa_heartbeat_success_")
                    .last()
                    .unwrap()
                    .to_string(),
            ));
        }
    }
    // heartbeat failed
    Ok(None)
}

pub async fn scan_local_debug_host() -> Result<HashMap<String, String>, Box<dyn std::error::Error>>
{
    let local_host = format!("http://{}", DEFAULT_HOSTNAME);
    // from port 9081 to 9091, scan all ports
    let mut hosts = HashMap::new();
    for port in 9081..9092 {
        let url = format!("{}:{}", local_host, port);
        let resp = heartbeat(&url).await?;
        if let Some(resp) = resp {
            hosts.insert(resp, url.to_string());
        }
    }
    // log::info!("scan_local_debug_host: {:?}", hosts);
    Ok(hosts)
}
