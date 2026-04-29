use dirs::home_dir;

use super::*;

#[test]
fn test_data_dir_path() {
    let home_dir = home_dir().expect("Should be able to compute home directory");
    // ChannelState, by default, is configured for Channel::Oss.
// Vesper white-label change — safe, no UI impact
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            assert_eq!(data_dir(), home_dir.join(".vesper"));
        } else if #[cfg(target_os = "linux")] {
            assert_eq!(data_dir(), home_dir.join(".local/share/vesper"));
        } else if #[cfg(windows)] {
            assert_eq!(data_dir(), home_dir.join("AppData\\Roaming\\vesper\\Vesper\\data"));
        } else {
            unimplemented!("Need to update tests for current platform!");
        }
    }
}

#[test]
fn test_config_local_dir_path() {
    let home_dir = home_dir().expect("Should be able to compute home directory");
    // ChannelState, by default, is configured for Channel::Oss.
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            assert_eq!(config_local_dir(), home_dir.join(".vesper"));
        } else if #[cfg(target_os = "linux")] {
            assert_eq!(config_local_dir(), home_dir.join(".config/vesper"));
        } else if #[cfg(windows)] {
            assert_eq!(config_local_dir(), home_dir.join("AppData\\Local\\vesper\\Vesper\\config"));
        } else {
            unimplemented!("Need to update tests for current platform!");
        }
    }
}

// Vesper white-label change — safe, no UI impact
#[test]
fn test_vesper_home_config_dir_path() {
    let home_dir = home_dir().expect("Should be able to compute home directory");
    let expected_dir_name = match ChannelState::data_profile() {
        Some(data_profile) => format!(".vesper-{data_profile}"),
        None => ".vesper".to_string(),
    };

    assert_eq!(
        warp_home_config_dir(),
        Some(home_dir.join(expected_dir_name))
    );
}

#[test]
fn test_vesper_home_skills_and_mcp_paths() {
    let Some(config_dir) = warp_home_config_dir() else {
        panic!("Should be able to compute Vesper home config directory");
    };

    assert_eq!(warp_home_skills_dir(), Some(config_dir.join("skills")));
    assert_eq!(
        warp_home_mcp_config_file_path(),
        Some(config_dir.join(".mcp.json"))
    );
}
#[test]
fn test_cache_dir_path() {
    let home_dir = home_dir().expect("Should be able to compute home directory");
    // ChannelState, by default, is configured for Channel::Oss.
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            assert_eq!(cache_dir(), home_dir.join("Library/Application Support/dev.vesper.Vesper"));
        } else if #[cfg(target_os = "linux")] {
            assert_eq!(cache_dir(), home_dir.join(".cache/vesper"));
        } else if #[cfg(windows)] {
            assert_eq!(cache_dir(), home_dir.join("AppData\\Local\\vesper\\Vesper\\cache"));
        } else {
            unimplemented!("Need to update tests for current platform!");
        }
    }
}

#[test]
fn test_state_dir_path() {
    let home_dir = home_dir().expect("Should be able to compute home directory");
    cfg_if::cfg_if! {
        // ChannelState, by default, is configured for Channel::Oss.
        if #[cfg(target_os = "macos")] {
            assert_eq!(state_dir(), home_dir.join("Library/Application Support/dev.vesper.Vesper"));
        } else if #[cfg(target_os = "linux")] {
            assert_eq!(state_dir(), home_dir.join(".local/state/vesper"));
        } else if #[cfg(windows)] {
            assert_eq!(state_dir(), home_dir.join("AppData\\Local\\vesper\\Vesper\\data"));
        } else {
            unimplemented!("Need to update tests for current platform!");
        }
    }
}

#[test]
fn test_project_path_for_warp_app_id() {
// Vesper white-label change — safe, no UI impact
    let project_dirs = project_dirs_for_app_id(AppId::new("dev", "vesper", "Vesper"), None)
        .expect("should be able to compute project dirs");
// Vesper white-label change — safe, no UI impact
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            assert_eq!(project_dirs.project_path(), "dev.vesper.Vesper");
        } else if #[cfg(target_os = "linux")] {
            assert_eq!(project_dirs.project_path(), "vesper");
        } else if #[cfg(windows)] {
            assert_eq!(project_dirs.project_path(), "vesper\\Vesper");
        } else {
            unimplemented!("Need to update tests for current platform!");
        }
    }
}

#[test]
fn test_project_path_for_warp_dev_app_id() {
// Vesper white-label change — safe, no UI impact
    let project_dirs = project_dirs_for_app_id(AppId::new("dev", "vesper", "VesperDev"), None)
        .expect("should be able to compute project dirs");
// Vesper white-label change — safe, no UI impact
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            assert_eq!(project_dirs.project_path(), "dev.vesper.VesperDev");
        } else if #[cfg(target_os = "linux")] {
            assert_eq!(project_dirs.project_path(), "vesper-dev");
        } else if #[cfg(windows)] {
            assert_eq!(project_dirs.project_path(), "vesper\\VesperDev");
        } else {
            unimplemented!("Need to update tests for current platform!");
        }
    }
}

#[test]
fn test_project_path_for_oss_app_id() {
// Vesper white-label change — safe, no UI impact
    let project_dirs = project_dirs_for_app_id(AppId::new("dev", "vesper", "Vesper"), None)
        .expect("should be able to compute project dirs");
// Vesper white-label change — safe, no UI impact
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            assert_eq!(project_dirs.project_path(), "dev.vesper.Vesper");
        } else if #[cfg(target_os = "linux")] {
            assert_eq!(project_dirs.project_path(), "vesper");
        } else if #[cfg(windows)] {
            assert_eq!(project_dirs.project_path(), "vesper\\Vesper");
        } else {
            unimplemented!("Need to update tests for current platform!");
        }
    }
}
