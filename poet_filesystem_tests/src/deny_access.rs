use std::fs::Permissions;
use std::fs::set_permissions;
use std::io;
use std::os::unix::fs::PermissionsExt as _;
use std::path::Path;

const NO_PERMISSIONS: u32 = 0o000;

pub fn deny_access(path: &Path) -> io::Result<()> {
    set_permissions(path, Permissions::from_mode(NO_PERMISSIONS))
}
