use std::collections::HashMap;

use debug_print::debug_println;
use serde::{Deserialize, Serialize};

// {
//   "2FC4D5E0-B561-4984-BD4B-7C8459F587EA" : {
//     "build" : "21F79",
//     "deletable" : true,
//     "identifier" : "2FC4D5E0-B561-4984-BD4B-7C8459F587EA",
//     "kind" : "Disk Image",
//     "lastUsedAt" : "2024-10-24T11:01:41Z",
//     "mountPath" : "\/Library\/Developer\/CoreSimulator\/Volumes\/iOS_21F79",
//     "path" : "\/Library\/Developer\/CoreSimulator\/Images\/2FC4D5E0-B561-4984-BD4B-7C8459F587EA.dmg",
//     "platformIdentifier" : "com.apple.platform.iphonesimulator",
//     "runtimeBundlePath" : "\/Library\/Developer\/CoreSimulator\/Volumes\/iOS_21F79\/Library\/Developer\/CoreSimulator\/Profiles\/Runtimes\/iOS 17.5.simruntime",
//     "runtimeIdentifier" : "com.apple.CoreSimulator.SimRuntime.iOS-17-5",
//     "signatureState" : "Verified",
//     "sizeBytes" : 7336630623,
//     "state" : "Ready",
//     "version" : "17.5"
//   }
// }

// all fields are optional
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Runtime {
    pub identifier: String,
    pub build: String,
    pub version: String,
    pub state: String,
    pub deletable: bool,
    pub kind: String,
    // optional
    #[serde(rename = "lastUsedAt")]
    pub last_used_at: Option<String>,
    #[serde(rename = "mountPath")]
    pub mount_path: Option<String>,
    pub path: Option<String>,
    #[serde(rename = "platformIdentifier")]
    pub platform_identifier: Option<String>,
    #[serde(rename = "runtimeBundlePath")]
    pub runtime_bundle_path: Option<String>,
    #[serde(rename = "runtimeIdentifier")]
    pub runtime_identifier: String,
    #[serde(rename = "signatureState")]
    pub signature_state: Option<String>,
    #[serde(rename = "sizeBytes")]
    pub size_bytes: u64,
    #[serde(rename = "unusableErrorMessage")]
    pub unusable_error_message: Option<String>,
}
