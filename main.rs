use serde::Deserialize;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{chown, PermissionsExt};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;
use users::{get_group_by_name, get_user_by_name};

#[derive(Deserialize)]
struct FileMeta {
    path: String,
    owner: Option<String>, // user name or UID
    group: Option<String>, // group name or GID
    mode: Option<String>,  // octal number
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut created_files = Vec::new();

    for (key, _value) in env::vars() {
        if key.starts_with("ENV2FILE_") && key.ends_with("_CONTENT") {
            let id = key
                .trim_start_matches("ENV2FILE_")
                .trim_end_matches("_CONTENT");
            let content = env::var(format!("ENV2FILE_{}_CONTENT", id))?;
            let meta_str = env::var(format!("ENV2FILE_{}_META", id))?;

            let meta: FileMeta = serde_json::from_str(&meta_str)?;

            if meta.path.is_empty() {
                eprintln!("Path not specified for ID {}", id);
                continue;
            }

            // create file

            if let Some(parent) = Path::new(&meta.path).parent() {
                fs::create_dir_all(parent)?;
            }

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&meta.path)?;
            file.write_all(content.as_bytes())?;

            // set owner and group

            let owner_id = if let Some(owner) = meta.owner {
                if let Ok(uid) = owner.parse::<u32>() {
                    Some(uid)
                } else {
                    match get_user_by_name(&owner) {
                        Some(user) => Some(user.uid()),
                        None => {
                            eprintln!("User not found: {}", owner);
                            None
                        }
                    }
                }
            } else {
                None
            };

            let group_id = if let Some(group) = meta.group {
                if let Ok(gid) = group.parse::<u32>() {
                    Some(gid)
                } else {
                    match get_group_by_name(&group) {
                        Some(group) => Some(group.gid()),
                        None => {
                            eprintln!("Group not found: {}", group);
                            None
                        }
                    }
                }
            } else {
                None
            };

            chown(&meta.path, owner_id, group_id)?;

            // set mode

            let mode = if let Some(mode_str) = meta.mode {
                if let Ok(mode) = u32::from_str_radix(&mode_str, 8) {
                    mode
                } else {
                    return Err(format!("Invalid mode: {}", mode_str).into());
                }
            } else {
                0o644
            };

            fs::set_permissions(&meta.path, fs::Permissions::from_mode(mode))?;

            created_files.push(meta.path);
        }
    }

    println!("Created {} files.", created_files.len());
    for file in created_files {
        println!("- {}", file);
    }

    if let Some(command) = std::env::args().nth(1) {
        let error = Command::new(command).args(std::env::args().skip(2)).exec();
        eprintln!("Failed to execute command: {}", error);
    }

    Ok(())
}
