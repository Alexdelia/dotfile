pub enum FileType {
    File,
    Directory,
    Symlink(Symlink),
    None,
}

enum Symlink {
    TargetExist,
    TargetNotExist,
}
