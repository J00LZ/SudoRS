extern crate clap;
extern crate nix;
extern crate pwhash;
extern crate shadow;
extern crate users;

use std::ffi::{CStr, CString};

use clap::{App, AppSettings, Arg};
use nix::unistd::*;

mod runner;

fn main() {
    let res = App::new("SudoRS")
        .setting(AppSettings::TrailingVarArg)
        .version("1.0")
        .author("Julius de Jeu <julius@voidcorp.nl>")
        .about("A simple sudo implementation in Rust")
        .arg(Arg::with_name("user").short("u").long("user").takes_value(true).default_value("root").help("The user to run the program as"))
        .arg(Arg::with_name("program").required(true).multiple(true).help("The program to execute with arguments"))
        .get_matches();
    let prog = res.value_of("program").unwrap();
    let pa = res.values_of_lossy("program").unwrap();
    let user = res.value_of("user").unwrap();
    let mut p: Vec<&str> = pa.iter().map(|s| &**s).collect();
    p.drain(0..1);
    let orig_id = nix::unistd::getuid();
    let uid = users::get_user_by_uid(orig_id.as_raw()).unwrap();
    let hash = shadow::Shadow::from_name(uid.name().to_str().unwrap()).unwrap();
    let pass = rpassword::prompt_password_stdout("Password: ").unwrap();
    let correct = pwhash::unix::verify(pass.as_str(), &hash.password);

    if correct {
        let actual = users::get_user_by_name(user).expect("Invalid username");
        initgroups(
            CString::new(user).unwrap().as_c_str(),
            Gid::from_raw(actual.primary_group_id()),
        ).expect("Could not set additional groups");
        setgid(Gid::from_raw(actual.primary_group_id())).expect("Could not set GID");
        setuid(Uid::from_raw(actual.uid())).expect("Could not set UID");

        runner::execute(prog, &p);
    } else {
        eprintln!("Invalid Password for user {}", &hash.name)
    }
}
