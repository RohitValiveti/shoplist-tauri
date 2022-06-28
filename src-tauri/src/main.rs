
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
async fn open_add_win(handle: tauri::AppHandle){
  let win = tauri::WindowBuilder::new(
    &handle,
    "add", tauri::WindowUrl::App("addItem.html".into())
  ).build().unwrap();
}

fn main() {
  let context = tauri::generate_context!();
    use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let add_item = CustomMenuItem::new("add item".to_string(), "Add item");
    let clear_items = CustomMenuItem::new("clear items".to_string(), "Clear items");
    let file = Submenu::new("File", Menu::new().add_item(add_item).add_item(clear_items).add_item(quit));

    let toggle_dev = CustomMenuItem::new("toggle dev".to_string(), "Toggle DevTools");
    let refresh = CustomMenuItem::new("refresh".to_string(), "Refresh");
    let dev_tools = Submenu::new("Developer Tools", Menu::new().add_item(toggle_dev).add_item(refresh));

    let menu = Menu::new()
      .add_native_item(MenuItem::Copy)
      .add_submenu(file)
      .add_submenu(dev_tools);

      
      tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event|{
          match event.menu_item_id(){
            "quit" => {
              std::process::exit(0);
            }
            "add item" => {
              open_add_win(event);
            }
            "clear items" => {
              
            }
            "toggle dev" => {
              
            }
            "refresh" => {
              
            }
            _ => {}
          }
        })
        .run(context)
        .expect("error while running tauri application");
}
