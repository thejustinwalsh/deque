use std::env::consts;

use tauri::{
    AboutMetadata, CustomMenuItem, Manager, Menu, MenuItem, Submenu, WindowMenuEvent, Wry,
};

pub fn get() -> Menu {
    match consts::OS {
        "macos" => menu_bar(),
        _ => Menu::new(),
    }
}

fn menu_bar() -> Menu {
    let app_menu = Menu::new()
        .add_native_item(MenuItem::About("Deque".to_string(), AboutMetadata::new()))
        .add_native_item(MenuItem::Separator)
        .add_item(
            CustomMenuItem::new("open_settings".to_string(), "Settings...")
                .accelerator("CmdOrCtrl+Comma"),
        )
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Services)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Hide)
        .add_native_item(MenuItem::HideOthers)
        .add_native_item(MenuItem::ShowAll)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Quit);

    let file_menu = Menu::new().add_item(
        CustomMenuItem::new("close".to_string(), "Close Window").accelerator("CmdOrCtrl+W"),
    );

    let edit_menu = Menu::new()
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::SelectAll);

    let view_menu = Menu::new().add_native_item(MenuItem::EnterFullScreen);

    let window_menu = Menu::new()
        .add_native_item(MenuItem::Minimize)
        .add_native_item(MenuItem::Zoom)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::ShowAll);

    #[cfg(debug_assertions)]
    let view_menu = {
        view_menu
            .add_native_item(MenuItem::Separator)
            .add_item(
                CustomMenuItem::new("reload_app".to_string(), "Reload").accelerator("CmdOrCtrl+R"),
            )
            .add_native_item(MenuItem::Separator)
            .add_item(
                CustomMenuItem::new("toggle_devtools".to_string(), "Toggle DevTools")
                    .accelerator("CmdOrCtrl+Alt+I"),
            )
            .add_item(
                CustomMenuItem::new(
                    "toggle_rq_devtools".to_string(),
                    "Toggle ReactQuery DevTools",
                )
                .accelerator("CmdOrCtrl+Alt+R"),
            )
    };

    Menu::new()
        .add_submenu(Submenu::new("Deque", app_menu))
        .add_submenu(Submenu::new("File", file_menu))
        .add_submenu(Submenu::new("Edit", edit_menu))
        .add_submenu(Submenu::new("View", view_menu))
        .add_submenu(Submenu::new("Window", window_menu))
}

pub fn handle_menu_event(event: WindowMenuEvent<Wry>) {
    match event.menu_item_id() {
        "quit" => {
            let app = event.window().app_handle();
            app.exit(0);
        }
        "open_settings" => event.window().emit("keybind", "open_settings").unwrap(),
        "close" => {
            let window = event.window();

            #[cfg(debug_assertions)]
            if window.is_devtools_open() {
                window.close_devtools();
            } else {
                window.close().unwrap();
            }

            #[cfg(not(debug_assertions))]
            window.close().unwrap();
        }
        #[cfg(debug_assertions)]
        "reload_app" => event.window().emit("keybind", "reload_app").unwrap(),
        #[cfg(debug_assertions)]
        "toggle_devtools" => {
            let window = event.window();

            if window.is_devtools_open() {
                window.close_devtools();
            } else {
                window.open_devtools();
            }
        }
        #[cfg(debug_assertions)]
        "toggle_rq_devtools" => event
            .window()
            .emit("keybind", "toggle_rq_devtools")
            .unwrap(),
        _ => {}
    }
}
