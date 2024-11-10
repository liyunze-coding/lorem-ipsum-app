mod lipsum;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn generate_ipsum(paragraphs: u32) -> String {
    let res = lipsum::get_ipsum(paragraphs).await;

    res.unwrap().get_text().to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, generate_ipsum])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
