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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_url])
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
