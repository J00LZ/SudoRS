extern crate clap;
extern crate nix;
extern crate pwhash;
extern crate shadow;
extern crate users;

use std::collections::HashMap;
use std::ffi::CString;
use std::time::Duration;

use clap::{App, AppSettings, Arg};
use nix::unistd::*;

use crate::config::Override;

mod runner;
mod config;

struct ProgramArgs<'a> {
    user: &'a str,
    executable: &'a str,
    params: Vec<&'a str>,
}

fn main() {
    let res = App::new("SudoRS")
        .setting(AppSettings::TrailingVarArg)
        .version("1.0")
        .author("Julius de Jeu <julius@voidcorp.nl>")
        .about("A simple sudo implementation in Rust")
        .arg(Arg::with_name("user").short("u").long("user").takes_value(true).default_value("root").help("The user to run the program as"))
        .arg(Arg::with_name("program").required(true).multiple(true).help("The program to execute with arguments"))
        .get_matches();

    let cfg = config::get_config().expect("Error creating config!");
//    println!("{:?}", cfg);
    let pa = res.values_of_lossy("program").unwrap();
    let mut p: Vec<&str> = pa.iter().map(|s| &**s).collect();
    p.drain(0..1);

    let args = ProgramArgs {
        user: res.value_of("user").unwrap(),
        executable: res.value_of("program").unwrap(),
        params: p,
    };

    let orig_id = nix::unistd::getuid();
    let uid = users::get_user_by_uid(orig_id.as_raw()).unwrap();

    let gs: Vec<String> = uid.groups().unwrap_or(vec![]).into_iter()
        .filter(|gr| gr.gid() != 0)
        .map(|group| group.name().to_os_string().into_string().expect("wat?"))
        .collect();

    let u = cfg.overrides.iter()
        .filter(|&(k, v)| (!v.is_group && k == uid.name().to_str().unwrap()))
        .collect::<HashMap<&String, &Override>>().get(&uid.name().to_str().unwrap().to_string()).map(|&o| o);
    let g = cfg.overrides.iter()
        .filter(|&(k, v)| (v.is_group && gs.contains(k)))
        .collect::<HashMap<&String, &Override>>();

    if !cfg.general.allow_all && u.is_none() && g.keys().len() == 0 {
        println!("User not in sudors file, this incident will be reported.");
        return;
    }

    let agr = uid.groups().unwrap_or(Vec::new());
    let mut grpa = &Override::default();
    for grp in agr {
        match g.get(&grp.name().to_str().unwrap().to_string()) {
            None => {},
            Some(a) => grpa = *a,
        }
    }

    let overr = u.unwrap_or(grpa);

    if !overr.runas.contains(&args.user.to_string()) && !overr.runas.is_empty() {
        println!("Cannot run as {}", args.user);
        return;
    }

    let cmd = quale::which(args.executable).expect("Command not found...");
    if !overr.allowed_commands.is_empty() && !overr.allowed_commands.contains(&cmd.to_str().expect("Failed to convert OSString to &str").to_string()) {
        println!("This command is not allowed!");
        return;
    }

    let hash = shadow::Shadow::from_name(uid.name().to_str().unwrap()).expect("oof?");

    let mut correct = false;
    for i in 0..cfg.general.retries {
        let pass = rpassword::prompt_password_stdout(cfg.general.prompt.replace("{}", uid.name().to_str().unwrap_or("oof")).as_str()).unwrap();
        correct = pwhash::unix::verify(pass.as_str(), &hash.password);
        if correct {
            break;
        }
        std::thread::sleep(Duration::from_secs(3));
        if i != cfg.general.retries - 1 {
            println!("Incorrect password, try again");
        }
    };

    if correct {
        let actual = users::get_user_by_name(args.user).expect("Invalid username");
        initgroups(
            CString::new(args.user).unwrap().as_c_str(),
            Gid::from_raw(actual.primary_group_id()),
        ).expect("Could not set additional groups");
        setgid(Gid::from_raw(actual.primary_group_id())).expect("Could not set GID");
        setuid(Uid::from_raw(actual.uid())).expect("Could not set UID");

        runner::execute(args.executable, &args.params);
    } else {
        eprintln!("Invalid Password for user {}", &hash.name)
    }
}
