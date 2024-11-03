use std::collections::HashMap;

/**
 * This module is used to execute the command of the simulator
 */
use debug_print::debug_println;
use log::info;

use super::device::{Device, DeviceMap};
use super::runtime::{Runtime};
pub fn get_all_devices() -> DeviceMap {
    // exec `xcrun simctl list --json devices` and parse the output
    let output = std::process::Command::new("xcrun")
        .arg("simctl")
        .arg("list")
        .arg("--json")
        .arg("devices")
        .arg("available")
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    let mut devices: DeviceMap = serde_json::from_str(&output).unwrap();
    // set os_version field in devices using the key of the hashmap
    for (key, device) in devices.devices.iter_mut() {
        device.iter_mut().for_each(|d| {
            d.os_version = Some(
                key.clone()
                    .split(".")
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap_or("unknown system version")
                    .to_string(),
            );
        });
    }
    devices
}

pub fn get_all_runtimes() -> HashMap<String, Runtime> {
    // exec `xcrun simctl list --json runtimes` and parse the output
    let output = std::process::Command::new("xcrun")
        .arg("simctl")
        .arg("runtime")
        .arg("list")
        .arg("--json")
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    let map: HashMap<String, Runtime> = serde_json::from_str(&output).unwrap_or_else(|_| HashMap::new());
    map
}

pub fn delete_runtime(app: tauri::AppHandle, id: String) -> Result<(), String> {
    debug_println!("delete simulator runtime: {}", id);
    // exec `xcrun simctl    delete the runtime
    let output = std::process::Command::new("xcrun")
        .arg("simctl")
        .arg("runtime")
        .arg("delete")
        .arg(id)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("{}", output);
    Ok(())
}


pub fn boot_device(udid: &str) {
    // exec `xcrun simctl boot <udid>` to boot the device
    let output = std::process::Command::new("xcrun")
        .arg("simctl")
        .arg("boot")
        .arg(udid)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("boot device {}", output);
}

pub fn shutdown_device(udid: &str) {
    // exec `xcrun simctl shutdown <udid>` to shutdown the device
    let output = std::process::Command::new("xcrun")
        .arg("simctl")
        .arg("shutdown")
        .arg(udid)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("{}", output);
}

pub fn erase_device(udid: &str) {
    // exec `xcrun simctl erase <udid>` to erase the device
    let output = std::process::Command::new("xcrun")
        .arg("simctl")
        .arg("erase")
        .arg(udid)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("{}", output);
}

pub fn install_app(udid: &str, app_path: &str) {
    // exec `xcrun simctl install <udid> <app_path>` to install the app
    let output = std::process::Command::new("xcrun")
        .arg("simctl")
        .arg("install")
        .arg(udid)
        .arg(app_path)
        .output()
        .expect("failed to install app");
    let output = String::from_utf8(output.stdout).unwrap();
    info!("install app: {}", output);
    println!("{}", output);
}

pub fn uninstall_app(udid: &str, bundle_id: &str) {
    // exec `xcrun simctl uninstall <udid> <bundle_id>` to uninstall the app
    let output = std::process::Command::new("xcrun")
        .arg("simctl")
        .arg("uninstall")
        .arg(udid)
        .arg(bundle_id)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("{}", output);
}

pub fn launch_app(udid: &str, bundle_id: &str) {
    // exec `xcrun simctl launch <udid> <bundle_id>` to launch the app
    let output = std::process::Command::new("xcrun")
        .arg("simctl")
        .arg("launch")
        .arg(udid)
        .arg(bundle_id)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("{}", output);
}

pub fn terminate_app(udid: &str, bundle_id: &str) {
    // exec `xcrun simctl terminate <udid> <bundle_id>` to terminate the app
    let output = std::process::Command::new("xcrun")
        .arg("simctl")
        .arg("terminate")
        .arg(udid)
        .arg(bundle_id)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("{}", output);
}

pub fn open_url(udid: &str, url: &str) {
    debug_println!("open url: {} {}", udid, url);
    // exec `xcrun simctl openurl <udid> <url>` to open the url
    let output = std::process::Command::new("xcrun")
        .arg("simctl")
        .arg("openurl")
        .arg(udid)
        .arg(url)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("{}", output);
}

pub fn list_apps(udid: &str) {
    // exec `xcrun simctl listapps <udid>` to list the apps
    let output = std::process::Command::new("xcrun")
        .arg("simctl")
        .arg("listapps")
        .arg(udid)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("{}", output);
}

/**
 * Find all menu items in Safari Dev Tool
 * output is a JSON array
 * try to activate Safari and close the front window
 */
fn find_all_menu_item_in_dev_tool() -> String {
    let apple_script = r#"
    tell application "Safari"
        activate
        try
            close front window
        end try
        tell application "System Events"
            tell process "Safari"
                -- Make sure the application is running and in the foreground
                set frontmost to true
                -- Wait a moment for the application to become the frontmost application
                -- delay 1
                
                -- Navigate the menu bar structure
                tell menu bar 1
                    tell menu bar item "Develop"
                        tell menu "Develop"
                            -- Initialize a string to build the JSON array
                            set jsonString to "["
                            -- Loop through each menu item
                            repeat with i from 1 to count of menu items
                                set aMenuItem to item i of menu items
                                try
                                    -- For each menu item, add its name to the JSON string
                                    set menuItemName to name of aMenuItem
                                    if i > 1 then
                                        set jsonString to jsonString & ", "
                                    end if
                                    set jsonString to jsonString & "\"" & menuItemName & "\""
                                on error errMsg
                                    -- Error handling
                                    log errMsg
                                end try
                            end repeat
                            -- Close the JSON array
                            set jsonString to jsonString & "]"
                            
                            -- Output the JSON string to stdout using a shell command
                            do shell script "echo " & quoted form of jsonString
                        end tell
                    end tell
                end tell
            end tell
        end tell
    end tell
    "#;
    // exec `osascript -e 'tell application "System Events" to tell process "Safari" to get entire contents of menu bar 1'` to find all menu items in Safari Dev Tool
    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(apple_script)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("{}", output);
    output
}

pub fn find_all_web_view_windows_in_simultor(simulator: &str) -> String {
    let apple_script = r#"

    "#
    .to_owned();
    // exec `osascript -e 'tell application "System Events" to tell process "Simulator" to get entire contents of menu bar 1'` to find all menu items in Safari Dev Tool
    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(apple_script)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("{}", output);
    output
}

pub fn open_safari_dev_tool(udid: &str, web_view_name: Option<&str>) {
    let simulator = "LM73L9FPFH\nmacOS 14.4.1".to_string();
    let window =
        web_view_name.unwrap_or("api-b-sinfonlinea.dcarapi.com — customer-profile-input.html");
    // var1: ${simulator}
    // var2: ${window}
    let apple_script = r#"
    on menu_click(mList)
	local appName, topMenu, r
	
	-- Validate our input
	if mList's length < 3 then error "Menu list is not long enough"
	
	-- Set these variables for clarity and brevity later on
	set {appName, topMenu} to (items 1 through 2 of mList)
	set r to (items 3 through (mList's length) of mList)
	
	-- This overly-long line calls the menu_recurse function with
	-- two arguments: r, and a reference to the top-level menu
	tell application "System Events" to my menu_click_recurse(r, ((process appName)'s ¬
		(menu bar 1)'s (menu bar item topMenu)'s (menu topMenu)))
    end menu_click

    on menu_click_recurse(mList, parentObject)
        local f, r
        
        -- `f` = first item, `r` = rest of items
        set f to item 1 of mList
        if mList's length > 1 then set r to (items 2 through (mList's length) of mList)
        
        -- either actually click the menu item, or recurse again
        tell application "System Events"
            if mList's length is 1 then
                click parentObject's menu item f
            else
                my menu_click_recurse(r, (parentObject's (menu item f)'s (menu f)))
            end if
        end tell
    end menu_click_recurse
    menu_click({"Safari", "Develop", "${simulator}", "${window}"})
    "#
    .to_owned()
    .replace("${simulator}", &simulator)
    .replace("${window}", window);

    // let all_menu_items = find_all_menu_item_in_dev_tool();
    // let all_menu_items: Vec<String> = serde_json::from_str(&all_menu_items).unwrap();
    // let all_devices = get_all_devices();
    // let device = all_devices.get_device_by_udid(udid).unwrap();
    // let simulator = device.name.clone();
    // let simulator = "LM73L9FPFH\nmacOS 14.4.1".to_string();
    // let window = web_view_name.unwrap_or("api-b-sinfonlinea.dcarapi.com — customer-profile-input.html");
    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(apple_script)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("{}", output);
}

pub fn open_simulator_app() {
    let output = std::process::Command::new("open")
        .arg("/Applications/Xcode.app/Contents/Developer/Applications/Simulator.app")
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8(output.stdout).unwrap();
    println!("open simulator app {}", output);
}
