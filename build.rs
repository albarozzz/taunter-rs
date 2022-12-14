use std::env;
use std::error::Error;
use winres::WindowsResource;

fn add_icon_to_bin_when_building_for_win(icon_path: &str) -> Result<(), Box<dyn Error>> {
    if env::var("CARGO_CFG_TARGET_FAMILY")? == "windows" {
        let mut res = WindowsResource::new();
        let target_env = std::env::var("CARGO_CFG_TARGET_ENV")?;
        match target_env.as_str() {
            "gnu" => res
                .set_ar_path("x86_64-w64-mingw32-ar")
                .set_windres_path("x86_64-w64-mingw32-windres")
                .set_toolkit_path(".")
                .set_icon(icon_path),
            "msvc" => res.set_icon(icon_path),
            _ => panic!("Unsupported env: {}", target_env),
        };
        res.compile()?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    add_icon_to_bin_when_building_for_win("./assets/taunter.ico")
}
