use crate::sotre::{self, CostaStoreWrapper};

#[derive(Default)]
pub struct MyState {
    s: std::sync::Mutex<String>,
    t: std::sync::Mutex<std::collections::HashMap<String, String>>,
}
// remember to call `.manage(MyState::default())`
#[tauri::command]
pub async fn call_app_method(state: tauri::State<'_, MyState>) -> Result<(), String> {
    *state.s.lock().unwrap() = "new string".into();
    state.t.lock().unwrap().insert("key".into(), "value".into());
    Ok(())
}

#[tauri::command]
pub async fn get_app_store(app: tauri::AppHandle) -> Result<Option<CostaStoreWrapper>, String> {
    Ok(sotre::get_tauri_store(app))
}
