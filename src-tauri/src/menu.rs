use log::info;
use serde::{Deserialize, Serialize};
use tauri::{
  AboutMetadata, AppHandle, CustomMenuItem, Manager, Menu, MenuItem, Runtime, Submenu,
  WindowMenuEvent,
};
use onekeepass_core::db_service as kp_service; 

#[allow(dead_code)]
pub mod menu_ids {
  pub const QUIT: &str = "Quit";
  pub const NEW_DATABASE: &str = "NewDatabase";
  pub const OPEN_DATABASE: &str = "OpenDatabase";
  pub const SAVE_DATABASE: &str = "SaveDatabase";
  pub const SAVE_DATABASE_AS: &str = "SaveDatabaseAs";
  pub const CLOSE_DATABASE: &str = "CloseDatabase";
  pub const LOCK_DATABASE: &str = "LockDatabase";
  pub const LOCK_ALL_DATABASES: &str = "LockAllDatabases";

  pub const SEARCH: &str = "Search";
  pub const NEW_ENTRY: &str = "NewEntry";
  pub const EDIT_ENTRY: &str = "EditEntry";
  pub const PASSWORD_GENERATOR: &str = "PasswordGenerator";
}
use menu_ids::*;

pub fn get_app_menu() -> Menu {
  let app_name = "OneKeePass";

  let quit = CustomMenuItem::new(QUIT, "Quit OneKeePass");

  #[allow(unused_mut)]
  let mut first_menu;
  #[cfg(target_os = "macos")]
  {
    let about = MenuItem::About(app_name.to_string(), AboutMetadata::default());
    first_menu = Menu::with_items([about.into(), MenuItem::Separator.into(), quit.into()]);
  }

  #[cfg(not(target_os = "macos"))]
  {
    first_menu = Menu::with_items([quit.into()]);
  }

  // In mac these are needed for Cut, Copy and Paste to work when user uses keyboard
  let edit_sub_menu = Menu::with_items([
    MenuItem::Cut.into(),
    MenuItem::Copy.into(),
    MenuItem::Paste.into(),
    MenuItem::Separator.into(),
    CustomMenuItem::new(SEARCH, "Search")
      .accelerator("CmdOrControl+F")
      .disabled()
      .into(),
  ]);

  let database_sub_menu = Menu::with_items([
    CustomMenuItem::new(NEW_DATABASE, "New Database")
      .accelerator("Shift+CmdOrControl+N")
      .into(),
    CustomMenuItem::new(OPEN_DATABASE, "Open Database")
      .accelerator("CmdOrControl+O")
      .into(),
    CustomMenuItem::new(SAVE_DATABASE, "Save Database")
      .accelerator("CmdOrControl+S")
      .disabled()
      .into(),
    CustomMenuItem::new(SAVE_DATABASE_AS, "Save Database As")
      .accelerator("CmdOrControl+S")
      .disabled()
      .into(),
    CustomMenuItem::new(CLOSE_DATABASE, "Close Database")
      .accelerator("CmdOrControl+W")
      .disabled()
      .into(),
    MenuItem::Separator.into(),
    CustomMenuItem::new(LOCK_DATABASE, "Lock Database")
      .accelerator("CmdOrControl+L")
      .disabled()
      .into(),
    CustomMenuItem::new(LOCK_ALL_DATABASES, "Lock All Databases")
      .accelerator("Shift+CmdOrControl+L")
      .disabled()
      .into(),
  ]);

  let entries_sub_menu = Menu::with_items([
    CustomMenuItem::new("NewEntry", "New Entry")
      .accelerator("CmdOrControl+N")
      .disabled()
      .into(),
    MenuItem::Separator.into(),
    CustomMenuItem::new(EDIT_ENTRY, "Edit Entry")
      .accelerator("CmdOrControl+E")
      .disabled()
      .into(),
  ]);

  let groups_sub_menu = Menu::with_items([]);

  let tools_sub_menu =
    Menu::with_items([
      CustomMenuItem::new(PASSWORD_GENERATOR, "Password Generator")
        .accelerator("CmdOrControl+G")
        .disabled()
        .into(),
    ]);

  #[cfg(target_os = "macos")]
  {
    Menu::new()
      .add_submenu(Submenu::new("App Menu", first_menu))
      .add_submenu(Submenu::new("Edit", edit_sub_menu))
      .add_submenu(Submenu::new("Database", database_sub_menu))
      .add_submenu(Submenu::new("Entries", entries_sub_menu))
      .add_submenu(Submenu::new("Groups", groups_sub_menu))
      .add_submenu(Submenu::new("Tools", tools_sub_menu))
  }

  #[cfg(not(target_os = "macos"))]
  {
    Menu::new()
      .add_submenu(Submenu::new("File", first_menu))
      .add_submenu(Submenu::new("Edit", edit_sub_menu))
      .add_submenu(Submenu::new("Database", database_sub_menu))
      .add_submenu(Submenu::new("Entries", entries_sub_menu))
      .add_submenu(Submenu::new("Groups", groups_sub_menu))
      .add_submenu(Submenu::new("Tools", tools_sub_menu))
  }
}

const TAURI_MENU_EVENT: &str = "TauriMenuEvent";

#[derive(Serialize, Deserialize, Debug)]
pub enum MenuAction {
  Close,
  Enable,
  Disable,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MenuActionRequest {
  menu_id: String,
  menu_action: MenuAction,
}

#[derive(Clone, serde::Serialize)]
/// Payload to send to the UI layer
struct MenuPayload {
  menu_id: String,
}

/// Called to act on some menu action or to enable/disable certain App menus based
/// on the UI state. Called in a tauri command from UI.
/// See 'onekeepass.frontend.background/menu-action-requested' ,
/// 'onekeepass.frontend.events.tauri-events/enable-app-menu'
pub fn menu_action_requested<R: Runtime>(request: MenuActionRequest, app: AppHandle<R>) {
  let menu_id = request.menu_id.as_str();
  match menu_id {
    QUIT => {
      info!("Quit requested from UI {:?}", request);
      let _r = kp_service::close_all_databases();
      app.exit(0);
    }
    EDIT_ENTRY | SAVE_DATABASE | SAVE_DATABASE_AS | LOCK_DATABASE | LOCK_ALL_DATABASES
    | CLOSE_DATABASE | PASSWORD_GENERATOR | SEARCH => {
      if let Some(main_window) = app.get_window("main") {
        let menu_handle = main_window.menu_handle();
        let t = match request.menu_action {
          MenuAction::Enable => true,
          MenuAction::Disable => false,
          _ => false,
        };
        if let Err(e) = menu_handle.get_item(menu_id).set_enabled(t) {
          log::error!(
            "Unexpectd error {:?} for the UI menu action {} ",
            e,
            menu_id
          );
        }
      }
    }
    _ => {
      info!("Not yet handled menu action: {:?}", request);
    }
  }
}

// This handles all menu events for any window ('main' window or any other window if used) for now
// All requested menu_id are just forwarded to the UI layer and UI layer decides what to do
// See functions in 'onekeepass.frontend.events.tauri-events' particularly 'handle-menu-events'
pub fn handle_menu_events(menu_event: &WindowMenuEvent) {
  menu_event
    .window()
    .emit(
      TAURI_MENU_EVENT,
      MenuPayload {
        menu_id: menu_event.menu_item_id().into(),
      },
    )
    .unwrap();
}
