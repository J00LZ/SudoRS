# SudoRS
> A (very good) implementation of sudo but it's in rust

## Building
1. Make sure you have cargo/rustc
2. `cargo build`
3. `chown root:root path/to/sudors` and `chmod u+s path/to/sudors`
4. Happy sudoing!

## Building (Arch)
1. Make sure `base-devel` is installed
2. Run `makepackage -si`, which syncs the dependencies, in this case cargo/rust, and installs the new package.
3. Happy sudoing!

## Configuration
In the file `/etc/sudors.toml` you can adjust the config, by default `root` can use all commands as any user 
without a password, and the `wheel` group can use any command as any user with a password. 