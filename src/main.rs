use std::time::Duration;

use chap13::{do_things, StderrLogger, VerbosityFilter};

use crate::chap19::chap19::{Language, PackageBuilder};

mod cahp8;
mod chap10;
mod chap12;
mod chap13;
mod chap19;


fn main() {
    let base64 = PackageBuilder::new("base64").version("0.13").build();
    println!("base64: {base64:?}");
    let log =
        PackageBuilder::new("log").version("0.4").language(Language::Rust).build();
    println!("log: {log:?}");
    let serde = PackageBuilder::new("serde")
        .authors(vec!["djmitche".into()])
        .version(String::from("4.0"))
        .dependency(base64.as_dependency())
        .dependency(log.as_dependency())
        .build();
    println!("serde: {serde:?}");
}