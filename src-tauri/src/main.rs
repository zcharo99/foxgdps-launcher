use tauri::Manager;
use std::process::{Command, ExitStatus};
use std::path::Path;
use std::env;

#[tauri::command]
fn get_home_dir() -> String {
    env::var("HOME").unwrap_or_else(|_| String::from("Not available"))
}

#[tauri::command]
fn run() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        // get the home directory
        let home_dir = env::var("HOME").expect("Unable to get HOME directory");

        // possible paths for Steam
        let steam_paths = [
            format!("{}/.steam/steam", home_dir),
            format!("{}/.local/share/Steam", home_dir),
            format!("{}/.var/app/com.valvesoftware.Steam/.steam/steam", home_dir),
        ];

        // flag to check if Proton is found
        let mut proton_found = false;
        let mut proton_path = String::new();

        for path in steam_paths.iter() {
            if Path::new(&path).exists() {
                let proton_executable_path = format!("{}/steamapps/common/Proton - Experimental/files/bin/wine64", path);
                if Path::new(&proton_executable_path).exists() {
                    proton_found = true;
                    proton_path = proton_executable_path;
                    break;
                }
            }
        }

        if !proton_found {
            return Err("Steam and Proton Experimental not found! Please install them first.".into());
        }

        // check if the executable path is valid
        let exe_path_str = format!("{}/FoxGDPS/FoxGDPS.exe", home_dir);
        let exe_path = Path::new(&exe_path_str);
        if !exe_path.exists() {
            return Err("FoxGDPS not found in your home directory, please install it /home/(username)/FoxGDPS".into());
        }

        // check if hackproldr.dll or Geode.dll exists in the executable's directory
        let mh_dll_path = exe_path.parent().unwrap_or_else(|| Path::new("")).join("hackpro.dll");
        let geode_dll_path = exe_path.parent().unwrap_or_else(|| Path::new("")).join("Geode.dll");

        if geode_dll_path.exists() {
            let dll_overrides = "xinput1_4=n,b";
            env::set_var("WINEDLLOVERRIDES", dll_overrides)
        } else if mh_dll_path.exists() {
            let dll_overrides = "XInput9_1_0=n,b";
            env::set_var("WINEDLLOVERRIDES", dll_overrides)
        }

        // run the executable with Proton and wait for it to finish
        Command::new(&proton_path)
            .envs(env::vars())
            .arg(&exe_path)
            .current_dir(exe_path.parent().unwrap()) // set the working directory
            .spawn()
            .map_err(|_| "Failed to execute process".to_string())?;
    }

    #[cfg(target_os = "windows")]
    {
        let exe_path = Path::new("C:\\Program Files\\FoxGDPS\\FoxGDPS.exe");
        // run the executable directly on Windows
        if !exe_path.exists() {
            return Err("FoxGDPS was not found. Please install FoxGDPS inside C:\\Program Files\\FoxGDPS".into());
        }

        Command::new(&exe_path)
            .spawn()
            .map_err(|_| "Failed to execute process".to_string())?
    }

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_home_dir, run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
