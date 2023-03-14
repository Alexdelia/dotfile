// need to know if file exist
// yes:
//   is it a symlink?
//   yes:
//     does it point to the same file?
//     yes:
//       do nothing
//     no:
//       update symlink
//   no:
//     bring up difference between file and file in data
//     create symlink depending on args
// no:
//   create symlink
