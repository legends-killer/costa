use crate::simulator::device::DeviceMap;


#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TrayMenu {
  pub simulator: DeviceMap,
}
impl TrayMenu {
    pub(crate) fn default() -> TrayMenu {
        TrayMenu {
            simulator: DeviceMap::default(),
        }
    }
}