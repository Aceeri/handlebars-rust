#![allow(unused_imports, dead_code)]
extern crate env_logger;
extern crate handlebars;
#[cfg(not(feature = "serde_type"))]
extern crate rustc_serialize;
#[macro_use]
extern crate maplit;

use std::path::Path;
use handlebars::Handlebars;

#[cfg(not(feature = "serde_type"))]
fn main() {
    env_logger::init().unwrap();
    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("template", &Path::new("./examples/template2.hbs")).ok().unwrap();
    handlebars.register_template_file("base0", &Path::new("./examples/base0.hbs")).ok().unwrap();
    handlebars.register_template_file("base1", &Path::new("./examples/base1.hbs")).ok().unwrap();

    let data0 = btreemap! {
        "title".to_string() => "example 0".to_string(),
        "parent".to_string() => "base0".to_string()
    };
    let data1 = btreemap! {
        "title".to_string() => "example 1".to_string(),
        "parent".to_string() => "base1".to_string()
    };

    println!("Page 0");
    println!("{}", handlebars.render("template", &data0).ok().unwrap());
    println!("=======================================================");

    println!("Page 1");
    println!("{}", handlebars.render("template", &data1).ok().unwrap());
}

#[cfg(feature = "serde_type")]
fn main() {}
