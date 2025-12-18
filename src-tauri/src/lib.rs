use std::process::Command;
use std::io::Write;
use std::fs;

#[derive(serde::Serialize)]
struct Printer {
    name: String,
    status: String,
}

#[tauri::command]
fn get_printers() -> Result<Vec<Printer>, String> {
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("lpstat")
            .arg("-p")
            .output()
            .map_err(|e| format!("Failed to execute lpstat: {}", e))?;

        if !output.status.success() {
            return Err("Failed to get printer list".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut printers = Vec::new();

        for line in stdout.lines() {
            if line.starts_with("printer ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let name = parts[1].to_string();
                    let status = if parts.len() >= 4 && parts[2] == "is" {
                        parts[3..].join(" ")
                    } else {
                        "unknown".to_string()
                    };
                    printers.push(Printer { name, status });
                }
            }
        }

        Ok(printers)
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("lpstat")
            .arg("-p")
            .output()
            .map_err(|e| format!("Failed to execute lpstat: {}", e))?;

        if !output.status.success() {
            return Err("Failed to get printer list".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut printers = Vec::new();

        for line in stdout.lines() {
            if line.starts_with("printer ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let name = parts[1].to_string();
                    let status = if parts.len() >= 4 && parts[2] == "is" {
                        parts[3..].join(" ")
                    } else {
                        "unknown".to_string()
                    };
                    printers.push(Printer { name, status });
                }
            }
        }

        Ok(printers)
    }

    #[cfg(target_os = "windows")]
    {
        let output = Command::new("wmic")
            .args(&["printer", "get", "name"])
            .output()
            .map_err(|e| format!("Failed to execute wmic: {}", e))?;

        if !output.status.success() {
            return Err("Failed to get printer list".to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut printers = Vec::new();

        for line in stdout.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() && trimmed != "Name" {
                printers.push(Printer {
                    name: trimmed.to_string(),
                    status: "ready".to_string(),
                });
            }
        }

        Ok(printers)
    }
}

#[tauri::command]
fn print_text(printer_name: String, text: String) -> Result<String, String> {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        let mut cmd = Command::new("lp");
        cmd.arg("-d").arg(&printer_name);

        let mut process = cmd
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start lp command: {}", e))?;

        if let Some(mut stdin) = process.stdin.take() {
            stdin
                .write_all(text.as_bytes())
                .map_err(|e| format!("Failed to write to stdin: {}", e))?;
        }

        let output = process
            .wait_with_output()
            .map_err(|e| format!("Failed to wait for lp command: {}", e))?;

        if output.status.success() {
            Ok("Print job sent successfully".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(format!("Print failed: {}", error))
        }
    }

    #[cfg(target_os = "windows")]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_file = std::env::temp_dir().join(format!("print_{}.txt", timestamp));
        fs::write(&temp_file, text)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;

        let output = Command::new("notepad")
            .arg("/p")
            .arg(temp_file.to_str().unwrap())
            .output()
            .map_err(|e| format!("Failed to print: {}", e))?;

        let _ = fs::remove_file(&temp_file);

        if output.status.success() {
            Ok("Print job sent successfully".to_string())
        } else {
            Err("Print failed".to_string())
        }
    }
}

#[tauri::command]
fn print_html(printer_name: String, html: String) -> Result<String, String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_file = std::env::temp_dir().join(format!("print_{}.html", timestamp));
        fs::write(&temp_file, html)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;

        let output = Command::new("lp")
            .arg("-d")
            .arg(&printer_name)
            .arg(temp_file.to_str().unwrap())
            .output()
            .map_err(|e| format!("Failed to execute lp command: {}", e))?;

        let _ = fs::remove_file(&temp_file);

        if output.status.success() {
            Ok("Print job sent successfully".to_string())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(format!("Print failed: {}", error))
        }
    }

    #[cfg(target_os = "windows")]
    {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_file = std::env::temp_dir().join(format!("print_{}.html", timestamp));
        fs::write(&temp_file, html)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;

        let output = Command::new("mshta")
            .arg(format!("javascript:window.print();close();"))
            .arg(temp_file.to_str().unwrap())
            .output()
            .map_err(|e| format!("Failed to print: {}", e))?;

        let _ = fs::remove_file(&temp_file);

        if output.status.success() {
            Ok("Print job sent successfully".to_string())
        } else {
            Err("Print failed".to_string())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![get_printers, print_text, print_html])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
