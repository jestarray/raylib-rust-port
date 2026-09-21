use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

const SDL_RELEASE_VERSION: &str = "3.4.16";

/// Checks if libSDL3.so exists in common Linux system library locations or via pkg-config.
fn is_sdl3_installed_on_system() -> bool {
    let standard_paths = [
        "/usr/lib",
        "/usr/local/lib",
        "/usr/lib64",
        "/usr/local/lib64",
        "/usr/lib/x86_64-linux-gnu",
        "/usr/lib/aarch64-linux-gnu",
        "/usr/lib/i386-linux-gnu",
        "/lib/x86_64-linux-gnu",
        "/lib/aarch64-linux-gnu",
    ];

    for path in &standard_paths {
        let p = Path::new(path);
        if p.join("libSDL3.so").exists() || p.join("libSDL3-3.0.so").exists() {
            return true;
        }
    }

    // Fallback: check if pkg-config knows about sdl3
    Command::new("pkg-config")
        .args(["--exists", "sdl3"])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    // Climb up from OUT_DIR (target/debug/build/pkg-hash/out) to reach the user's project root
    let consumer_project_root = out_path
        .parent() // out
        .and_then(|p| p.parent()) // pkg-hash
        .and_then(|p| p.parent()) // build
        .and_then(|p| p.parent()) // debug / release
        .and_then(|p| p.parent()); // target -> project root!

    // Check if the user opted out via feature flag
    let no_download_flag = env::var("CARGO_FEATURE_NO_DL_SDL").is_ok();

    // Check if SDL3 files already exist manually in the user's project root
    let manual_libs_in_root = consumer_project_root.and_then(|root| {
        if target_os == "windows" {
            let has_win = root.join("SDL3.lib").exists() && root.join("SDL3.dll").exists();
            if has_win {
                Some(root.to_path_buf())
            } else {
                None
            }
        } else {
            let has_unix = root.join("libSDL3.so").exists() || root.join("libSDL3.dylib").exists();
            if has_unix {
                Some(root.to_path_buf())
            } else {
                None
            }
        }
    });

    let is_linux_like = target_os == "linux" || target_os == "freebsd";
    let supports_download = target_os == "windows" || target_os == "android";
    let skip_download = no_download_flag || manual_libs_in_root.is_some() || !supports_download;

    let cache_dir = match out_path
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
    {
        Some(t) => t.join("sdl_cache"),
        None => out_path.join("sdl_cache"),
    };

    let mut bin_path_opt = None;

    if skip_download {
        if let Some(ref root) = manual_libs_in_root {
            println!(
                "cargo:warning=[SDL3] Detected existing SDL3 files in project root ({}).",
                root.display()
            );
            let dll_candidate = root.join("SDL3.dll");
            if dll_candidate.exists() {
                bin_path_opt = Some(dll_candidate);
            }
        } else if is_linux_like {
            let installed = is_sdl3_installed_on_system();
            if !installed {
                // Only print warnings if SDL3 is missing from the system
                println!(
                    "cargo:warning=[SDL3] Could not detect libSDL3.so in standard system paths or via pkg-config."
                );
                println!(
                    "cargo:warning=[SDL3] Please install SDL3 via your package manager (e.g., `sudo apt install libsdl3-dev` or `sudo dnf install libsdl3-devel`)."
                );
            }
        } else if no_download_flag {
            println!(
                "cargo:warning=[SDL3] 'no_dl_sdl' feature enabled. Skipping automatic download."
            );
            if let Some(root) = consumer_project_root {
                let dll_candidate = root.join("SDL3.dll");
                if dll_candidate.exists() {
                    bin_path_opt = Some(dll_candidate);
                }
            }
        } else {
            println!(
                "cargo:warning=[SDL3] Automatic SDL3 downloads are only supported for Windows and Android; skipping download for target OS '{}'.",
                target_os
            );
        }
    } else {
        // Automatic download and extraction is only supported for Windows and Android.
        let is_cached = cache_dir.exists() && cache_dir.join("lib").exists();

        if !is_cached {
            let _ = fs::create_dir_all(&cache_dir);

            let (url, archive_name, is_zip) = match target_os.as_str() {
                "windows" => {
                    if target_env == "msvc" {
                        (
                            format!(
                                "https://github.com/libsdl-org/SDL/releases/download/release-{}/SDL3-devel-{}-VC.zip",
                                SDL_RELEASE_VERSION, SDL_RELEASE_VERSION
                            ),
                            "sdl.zip",
                            true,
                        )
                    } else {
                        (
                            format!(
                                "https://github.com/libsdl-org/SDL/releases/download/release-{}/SDL3-devel-{}-mingw.tar.gz",
                                SDL_RELEASE_VERSION, SDL_RELEASE_VERSION
                            ),
                            "sdl.tar.gz",
                            false,
                        )
                    }
                }
                "android" => (
                    format!(
                        "https://github.com/libsdl-org/SDL/releases/download/release-{}/SDL3-devel-{}-android.zip",
                        SDL_RELEASE_VERSION, SDL_RELEASE_VERSION
                    ),
                    "sdl.zip",
                    true,
                ),
                _ => {
                    println!(
                        "cargo:warn= SDL3 gh repo does not have compiled .so/.lib/.a files on linux/other platforms, skipping..."
                    );
                    return;
                }
            };

            let archive_path = cache_dir.join(archive_name);

            println!(
                "cargo:warning=[SDL3] Downloading SDL3 release {}...",
                SDL_RELEASE_VERSION
            );
            let status = Command::new("curl")
                .args(["-L", "-o", &archive_path.to_string_lossy(), &url])
                .status()
                .expect("Failed to execute curl. Is curl installed?");
            if !status.success() {
                println!(
                    "cargo:warning=[SDL3] Download failed with status {}.",
                    status
                );
            }

            println!(
                "cargo:warning=[SDL3] Extracting archive directly into: {}",
                cache_dir.display()
            );

            let host_triple = env::var("HOST").unwrap();
            if is_zip {
                if host_triple.contains("windows") {
                    let status = Command::new("powershell")
                        .args([
                            "-Command",
                            &format!(
                                "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                                archive_path.display(),
                                cache_dir.display()
                            ),
                        ])
                        .status()
                        .expect("Failed to extract zip via PowerShell");
                    if !status.success() {
                        println!(
                            "cargo:warning=[SDL3] PowerShell extraction failed with status {}.",
                            status
                        );
                    }
                } else {
                    let status = Command::new("unzip")
                        .args([
                            "-q",
                            &archive_path.to_string_lossy(),
                            "-d",
                            &cache_dir.display().to_string(),
                        ])
                        .status()
                        .expect("Failed to execute unzip. Is unzip installed?");
                    if !status.success() {
                        println!(
                            "cargo:warning=[SDL3] unzip extraction failed with status {}.",
                            status
                        );
                    }
                }
            } else {
                let status = Command::new("tar")
                    .args([
                        "-xzf",
                        &archive_path.to_string_lossy(),
                        "-C",
                        &cache_dir.display().to_string(),
                    ])
                    .status()
                    .expect("Failed to execute tar. Is tar installed?");
                if !status.success() {
                    println!(
                        "cargo:warning=[SDL3] tar extraction failed with status {}.",
                        status
                    );
                }
            }

            // Flatten nested folder structure if present
            let nested_dir = cache_dir.join(format!("SDL3-{}", SDL_RELEASE_VERSION));
            if nested_dir.exists() && nested_dir.is_dir() {
                for entry in fs::read_dir(&nested_dir).unwrap().flatten() {
                    let dest = cache_dir.join(entry.file_name());
                    if dest.exists() {
                        if dest.is_dir() {
                            let _ = fs::remove_dir_all(&dest);
                        } else {
                            let _ = fs::remove_file(&dest);
                        }
                    }
                    let _ = fs::rename(entry.path(), dest);
                }
                let _ = fs::remove_dir_all(nested_dir);
            }
        }

        // Locate the runtime DLL for Windows. Android does not need a copied runtime file.
        bin_path_opt = if target_os == "windows" && target_env == "msvc" {
            let arch_folder = match target_arch.as_str() {
                "x86_64" => "x64",
                "x86" => "x86",
                "aarch64" => "arm64",
                _ => unreachable!(),
            };
            let lp = cache_dir.join("lib").join(arch_folder);
            let bin_candidates = [
                cache_dir.join("bin").join(arch_folder).join("SDL3.dll"),
                lp.join("SDL3.dll"),
            ];
            bin_candidates.into_iter().find(|p| p.exists())
        } else if target_os == "windows" {
            let dll = cache_dir.join("bin").join("SDL3.dll");
            dll.exists().then_some(dll)
        } else {
            None
        };
    }

    // Mirror DLL into target/debug or target/release if present (Windows)
    if let Some(bin_path) = bin_path_opt
        && bin_path.exists()
        && let Some(target_profile_dir) = out_path
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
    {
        let target_dest = target_profile_dir.join("SDL3.dll");
        if !target_dest.exists() {
            let _ = fs::copy(&bin_path, &target_dest);
        }
    }
}
