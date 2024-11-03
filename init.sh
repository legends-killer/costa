# init the constant_local.rs file
# if the directory does not exist, create it and add the content
if [ ! -d "src-tauri/src/constant_local.rs" ]; then
  mkdir -p src-tauri/src/constant_local.rs
  echo "pub const APP_DOWNLOAD_URL: &str = \"\";\npub const UPDATE_CHECK_URL: &str = \"\";" >> src-tauri/src/constant_local.rs
fi