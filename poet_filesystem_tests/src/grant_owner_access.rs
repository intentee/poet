use std::fs::Permissions;
use std::fs::set_permissions;
use std::io;
use std::os::unix::fs::PermissionsExt as _;
use std::path::Path;

const OWNER_READ_WRITE_EXECUTE: u32 = 0o700;

pub fn grant_owner_access(path: &Path) -> io::Result<()> {
    set_permissions(path, Permissions::from_mode(OWNER_READ_WRITE_EXECUTE))
}
