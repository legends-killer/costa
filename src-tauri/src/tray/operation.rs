use strum_macros::{Display, EnumString};

#[derive(EnumString, Debug, Display)]
pub enum OperationId {
    #[strum(to_string = "qr_code")]
    QrCode,
    #[strum(to_string = "safari_dev_tool")]
    Safari,
    #[strum(to_string = "install_app")]
    InstallApp,
    #[strum(to_string = "quit")]
    Quit,
    #[strum(to_string = "open_simulator")]
    OpenSimulator,
}

impl From<OperationId> for String {
    fn from(id: OperationId) -> Self {
        id.to_string()
    }
}

impl From<String> for OperationId {
    fn from(id: String) -> Self {
        match id.as_str() {
            "qr_code" => OperationId::QrCode,
            "safari_dev_tool" => OperationId::Safari,
            "install_app" => OperationId::InstallApp,
            "quit" => OperationId::Quit,
            _ => OperationId::OpenSimulator,
        }
    }
}
