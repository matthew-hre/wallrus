use std::path::Path;
use std::process::Command;

/// Set the GNOME desktop wallpaper using gsettings
pub fn set_gnome_wallpaper(image_path: &Path) -> Result<(), String> {
    let uri = format!(
        "file://{}",
        image_path
            .canonicalize()
            .map_err(|e| format!("Failed to resolve path: {}", e))?
            .display()
    );

    // Set for light mode
    run_gsettings("picture-uri", &uri)?;
    // Set for dark mode
    run_gsettings("picture-uri-dark", &uri)?;

    Ok(())
}

fn run_gsettings(key: &str, value: &str) -> Result<(), String> {
    let output = Command::new("gsettings")
        .args(["set", "org.gnome.desktop.background", key, value])
        .output()
        .map_err(|e| format!("Failed to run gsettings: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("gsettings failed for {}: {}", key, stderr));
    }

    Ok(())
}
