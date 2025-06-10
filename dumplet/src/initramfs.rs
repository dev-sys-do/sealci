use std::fs::{self, File};
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::errors::DumpletError;
use crate::transferred_file::{TransferPath, TransferredFile, TransferredFilePathIntoVm};

pub fn create_initramfs(
    rootfs_path: &Path,
    output_img: &Path,
    env: Option<Vec<&str>>,
    command: String,
    working_dir: PathBuf, // This is the directory where the init script will be executed
    transfer_files: Vec<String>,
    nameserver: Option<String>,
) -> Result<(), DumpletError> {
    let init_path = rootfs_path.join("init");
    if !transfer_files.is_empty() {
        for transfer_string in transfer_files {
            let transfer_path = TransferPath::try_from(transfer_string.clone())?;
            let transferred_path_vm =
                TransferredFilePathIntoVm::new(rootfs_path.to_path_buf(), transfer_path)?;
            TransferredFile::try_from(transferred_path_vm)?;
        }
    }

    let mut init_script = format!(
        r#"#!/bin/sh
        mount -t devtmpfs dev /dev
        mount -t proc proc /proc
        mount -t sysfs sysfs /sys
        ip link set up dev lo
        cd {}

        "#,
        working_dir.display()
    );

    if let Some(env_vars) = env {
        for var in env_vars {
            init_script.push_str(&format!("export {}\n", var));
        }
    }

    init_script.push_str(&format!(
        r#"
        {} &

        exec /sbin/getty -n -l /bin/sh 115200 /dev/console
        poweroff -f
    "#,
        command
    ));
    let mut file = File::create(&init_path)?;
    file.write_all(init_script.as_bytes())?;

    let mut perms = fs::metadata(&init_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&init_path, perms)?;

    // check if /etc exists
    if rootfs_path.join("etc").try_exists()? {
        let mut resolv_conf = File::create(rootfs_path.join("etc/resolv.conf"))?;
        match nameserver {
            Some(nameserver) => {
                resolv_conf.write_all(format!("nameserver {}\n", nameserver).as_bytes())?;
            }
            None => {
                resolv_conf.write_all(b"nameserver 8.8.8.8\n")?;
            }
        }
    }

    let find = Command::new("find")
        .arg(".")
        .arg("-print0")
        .current_dir(rootfs_path)
        .stdout(Stdio::piped())
        .spawn()?;

    let cpio = Command::new("cpio")
        .args([
            "--null",
            "--create",
            "--verbose",
            "--owner",
            "root:root",
            "--format=newc",
        ])
        .stdin(find.stdout.unwrap())
        .stdout(Stdio::piped())
        .current_dir(rootfs_path)
        .spawn()?;

    let mut xz = Command::new("xz")
        .args(["-9", "--format=lzma"])
        .stdin(cpio.stdout.unwrap())
        .stdout(File::create(output_img)?)
        .spawn()?;

    let status = xz.wait()?;
    if !status.success() {
        return Err(DumpletError::IoError(io::Error::new(
            io::ErrorKind::Other,
            "Failed to create initramfs image",
        )));
    }

    println!("🔹 Initramfs image created: {:?}", output_img);
    Ok(())
}
