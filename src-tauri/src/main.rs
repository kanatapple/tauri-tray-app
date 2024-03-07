use tauri::{
    async_runtime,
    menu::{MenuBuilder, MenuItemBuilder},
    path::BaseDirectory,
    tray::{ClickType, TrayIconBuilder},
    Icon, Manager,
};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app = app.app_handle().clone();
            async_runtime::spawn(async move {
                let toggle = MenuItemBuilder::with_id("toggle", "Toggle")
                    .build(&app)
                    .unwrap();
                let menu = MenuBuilder::new(&app).items(&[&toggle]).build().unwrap();
                let icon_path = app
                    .path()
                    .resolve("./icons/icon.ico", BaseDirectory::Resource)
                    .unwrap();
                let _ = TrayIconBuilder::new()
                    .menu(&menu)
                    .icon(Icon::File(icon_path))
                    .on_menu_event(move |_app, event| match event.id().as_ref() {
                        "toggle" => {
                            println!("toggle clicked");
                        }
                        _ => (),
                    })
                    .on_tray_icon_event(|_tray, event| {
                        if event.click_type == ClickType::Left {
                            println!("tray icon clicked");
                        }
                    })
                    .build(&app)
                    .unwrap();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
