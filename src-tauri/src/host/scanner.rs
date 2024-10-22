// This module is used to send the heartbeat to the dev host to indicate that the debugger is running.

use std::{collections::HashMap, time::Duration};

use debug_print::debug_println;

use crate::constant::{DEFAULT_HOSTNAME, DEFAULT_PATH};

pub async fn heartbeat(
    client: &reqwest::Client,
    url: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let url = format!("{}{}/heartbeat", url, DEFAULT_PATH);
    // TODO: send heartbeat to the dev host
    let resp = client.get(url).timeout(Duration::from_secs(1)).send().await;
    // log::info!("heartbeat resp: {}", resp);
    let resp = match resp {
        Ok(resp) => resp.text().await?,
        Err(e) => {
            return Err(Box::new(e));
        }
    };
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
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "heartbeat no response",
    )))
}

pub async fn scan_local_debug_host() -> Result<HashMap<String, String>, Box<dyn std::error::Error>>
{
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap();
    let local_host = format!("http://{}", DEFAULT_HOSTNAME);
    // from port 9081 to 9091, scan all ports
    let mut hosts = HashMap::new();
    for port in 9081..9092 {
        let url = format!("{}:{}", local_host, port);
        let resp = heartbeat(&client, &url).await;
        if let Ok(Some(resp)) = resp {
            hosts.insert(resp, url.to_string());
        }
    }
    // log::info!("scan_local_debug_host: {:?}", hosts);
    Ok(hosts)
}
