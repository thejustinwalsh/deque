#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod db;
mod routes;

use routes::Router;
use rspc::Config;
use std::{fs, path::PathBuf, sync::Arc};

#[tauri::command]
async fn ready(window: tauri::Window) {
    window.show().unwrap();
}

#[tokio::main]
async fn main() {
    let data_path = tauri::api::path::data_dir()
        .unwrap_or_else(|| PathBuf::from("./"))
        .join("dev.tjw.deque");
    fs::create_dir_all(&data_path).expect("Failed to create data directory");

    let db_path = data_path.join("db.sqlite");

    let client = Arc::new(
        db::new_client_with_url(
            format!("file:{}", db_path.to_str().unwrap_or("db.sqlite")).as_str(),
        )
        .await
        .expect("Failed to create database client"),
    );

    client
        ._db_push()
        .await
        .expect("Failed to initialize database");

    let router = Router::new()
        .config(
            Config::new()
                .export_ts_bindings("../src/hooks/rspc/bindings.ts")
                .set_ts_bindings_header("/* eslint-disable */"),
        )
        .merge("collection.", routes::collections::mount())
        .merge("item.", routes::items::mount())
        .build()
        .arced();

    tauri::Builder::default()
        .setup(|_app| Ok(()))
        .plugin(rspc::integrations::tauri::plugin(router, move || {
            client.clone()
        }))
        .invoke_handler(tauri::generate_handler![ready])
        .menu(app::menu::get())
        .on_menu_event(app::menu::handle_menu_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
