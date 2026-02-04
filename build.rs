use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=MACOSX_DEPLOYMENT_TARGET");
    println!("cargo:rerun-if-env-changed=PROFILE");
    println!("cargo:rerun-if-env-changed=TARGET");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // macOS-specific linker flags (optional, for compatibility or Swift/Objective-C support)
    if target_os == "macos" {
        let deployment_target =
            env::var("MACOSX_DEPLOYMENT_TARGET").unwrap_or_else(|_| "10.15".to_string());
        println!(
            "cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET={}",
            deployment_target
        );

        // Enable Swift runtime (if you use Swift code or async)
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

        // Link Objective-C runtime (required if using Obj-C classes or categories)
        println!("cargo:rustc-link-arg=-Wl,-ObjC");

        // Optional: Weak-link frameworks if you use screen recording APIs
        // println!("cargo:rustc-link-arg=-Wl,-weak_framework,ReplayKit");
        // println!("cargo:rustc-link-arg=-Wl,-weak_framework,ScreenCaptureKit");
    }

    // Windows resource compilation (icon, metadata)
    #[cfg(target_os = "windows")]
    if target_os == "windows" {
        #[cfg(target_env = "msvc")]
        {
            // Increase stack size to avoid stack overflow in some cases
            println!("cargo:rustc-link-arg=/stack:{}", 8 * 1024 * 1024);
        }

        let icon_path = "resources/windows/app-icon.ico";
        let icon = Path::new(icon_path);

        // Re-run build script if icon file changes
        println!("cargo:rerun-if-changed={}", icon.display());

        let mut res = winresource::WindowsResource::new();
        res.set_icon(icon.to_str().unwrap());
        res.set("FileDescription", "irgen");
        res.set("ProductName", "irgen");

        // Optional: uncomment to set version or copyright
        // res.set("FileVersion", "1.0.0.0");
        res.set("LegalCopyright", "© 2025 BeriBeli");

        if let Err(e) = res.compile() {
            eprintln!("Error compiling Windows resource: {}", e);
            std::process::exit(1);
        }
    }

    // Expose build target (e.g., x86_64-pc-windows-msvc) to runtime
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_string())
    );

    // Hardcode release channel — you only have one: "release"
    println!("cargo:rustc-env=RELEASE_CHANNEL=release");

    // Embed git commit SHA for build identification (optional)
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let git_dir = Path::new(&manifest_dir).join(".git");
        let head_path = git_dir.join("HEAD");

        if head_path.exists() {
            println!("cargo:rerun-if-changed={}", head_path.display());
            if let Ok(head_contents) = fs::read_to_string(&head_path)
        && let Some(ref_path) = head_contents.strip_prefix("ref: ").map(str::trim)
    {
        let ref_file = git_dir.join(ref_path);
        if ref_file.exists() {
            println!("cargo:rerun-if-changed={}", ref_file.display());
        }
    }
        }
    }
    if let Ok(output) = Command::new("git").args(["rev-parse", "HEAD"]).output()
        && output.status.success()
    {
        let git_sha = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("cargo:rustc-env=IRGEN_COMMIT_SHA={}", git_sha);

        // Only show warning in release builds to avoid noise during development
        if let Ok(profile) = std::env::var("PROFILE")
            && profile == "release"
        {
            println!(
                "cargo:warning=Info: using git commit '{}' for IRGEN_COMMIT_SHA",
                git_sha
            );
        }
    } else {
        // Fallback if git is unavailable
        println!("cargo:rustc-env=IRGEN_COMMIT_SHA=unknown");
    }
}
