use strum_macros::{Display, EnumString};

#[derive(EnumString, Debug, Display)]
pub enum OperationId {
    #[strum(to_string = "none")]
    None,
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
    #[strum(to_string = "select_host")]
    SelectHost,
    #[strum(to_string = "op_route_back")]
    RouteBack,
    #[strum(to_string = "op_route_forward")]
    RouteForward,
    #[strum(to_string = "op_route_refresh")]
    RouteRefresh,
    #[strum(to_string = "op_set_ppe")]
    SetPPE,
    #[strum(to_string = "op_set_boe")]
    SetBOE,
    #[strum(to_string = "op_login")]
    Login,
    #[strum(to_string = "op_logout")]
    Logout,
    #[strum(to_string = "op_debug_menu")]
    DebugMenu,
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
            // match id start with "open_simulator" by regex
            _ if id.starts_with(OperationId::OpenSimulator.to_string().as_str()) => {
                let udid = id
                    .split(OperationId::OpenSimulator.to_string().as_str())
                    .last()
                    .unwrap();
                OperationId::OpenSimulator
            }
            _ if id.starts_with(OperationId::SelectHost.to_string().as_str()) => {
                let host = id
                    .split(OperationId::SelectHost.to_string().as_str())
                    .last()
                    .unwrap();
                OperationId::SelectHost
            }
            "op_route_back" => OperationId::RouteBack,
            "op_route_forward" => OperationId::RouteForward,
            "op_route_refresh" => OperationId::RouteRefresh,
            "op_set_ppe" => OperationId::SetPPE,
            "op_set_boe" => OperationId::SetBOE,
            "op_login" => OperationId::Login,
            "op_logout" => OperationId::Logout,
            "op_debug_menu" => OperationId::DebugMenu,
            _ => OperationId::None,
        }
    }
}
