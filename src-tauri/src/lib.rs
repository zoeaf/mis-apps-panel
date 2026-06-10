use tauri::{WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
fn open_url(url: String) {
    if url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("file:///")
    {
        let _ = open::that(url);
    }
}

#[tauri::command]
fn run_cmd(cmd: String) -> Result<(), String> {
    if cmd.trim().is_empty() {
        return Ok(());
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        std::process::Command::new("cmd")
            .args(["/C", cmd.as_str()])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW — sin consola visible
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(not(windows))]
    {
        std::process::Command::new("sh")
            .args(["-c", cmd.as_str()])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_url, run_cmd])
        .setup(|app| {
            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::App("panel-apps.html".into()),
            )
            .title("Mis Apps")
            .inner_size(1200.0, 800.0)
            .resizable(true)
            // Intercepta los clics en links target="_blank" y los abre en el
            // navegador del sistema en lugar de crear una nueva ventana Tauri.
            .initialization_script(
                r#"
                document.addEventListener('click', function (e) {
                    var a = e.target.closest('a');
                    if (a && a.href && a.target === '_blank') {
                        e.preventDefault();
                        e.stopImmediatePropagation();
                        window.__TAURI_INTERNALS__.invoke('open_url', { url: a.href });
                    }
                }, true);
                "#,
            )
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Mis Apps");
}
