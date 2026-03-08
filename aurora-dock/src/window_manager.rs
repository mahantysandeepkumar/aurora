// src/window_manager.rs
use zbus::blocking::Connection;

pub fn is_app_running(app_id: &str) -> bool {
    println!("DEBUG: Checking for {}...", app_id);

    // Use the BLOCKING connection
    let Ok(conn) = Connection::session() else {
        return false;
    };

    // Use a direct blocking call
    let reply: zbus::Result<String> = conn
        .call_method(
            Some("org.kde.KWin"),
            "/KWin",
            Some("org.kde.KWin"),
            "supportInformation",
            &(),
        )
        .and_then(|m| m.body().deserialize());

    match reply {
        Ok(info) => {
            let found = info.to_lowercase().contains(&app_id.to_lowercase());
            if found {
                println!("   [!] {} is active", app_id);
            }
            found
        }
        Err(_) => false,
    }
}
