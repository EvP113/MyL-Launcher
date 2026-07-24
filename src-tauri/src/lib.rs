mod commands;
mod instances;
mod accounts;
mod versions;
mod launch;
mod modplatform;
mod import_mod;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_instances,
            commands::create_instance,
            commands::update_instance_config,
            commands::delete_instance,
            commands::get_instance_config,
            commands::list_accounts,
            commands::add_account_offline,
            commands::remove_account,
            commands::set_active_account,
            commands::get_account,
            commands::login_msa_pkce,
            commands::start_msa_auth,
            commands::poll_msa_auth,
            commands::refresh_msa_account,
            commands::launch_instance,
            commands::stop_game,
            commands::stop_game_by_pid,
            commands::open_folder,
            commands::ensure_java,
            commands::search_mods,
            commands::get_mod_versions,
            commands::download_mod,
            commands::list_installed_mods,
            commands::toggle_mod,
            commands::delete_mod,
            commands::add_mod_file,
            commands::get_mod_loaders,
            commands::get_mc_versions,
            commands::import_instance,
            commands::create_shortcut,
            commands::open_file_dialog,
            commands::search_modpacks,
            commands::get_modpack_versions,
            commands::import_modpack_from_url,
            commands::set_instance_icon,
            commands::list_resourcepacks,
            commands::list_shaderpacks,
            commands::list_worlds,
            commands::import_world_zip,
            commands::list_servers,
            commands::delete_resourcepack,
            commands::delete_shaderpack,
            commands::delete_world,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
