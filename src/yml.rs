extern crate yaml_rust;
use std::fs::File;
use std::io::prelude::*;
use yaml_rust::{YamlEmitter, YamlLoader};

pub fn print_yml(doc: &yaml_rust::Yaml) {
    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);
}

pub fn load_yml_configurations(file: &str) {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];
    print_yml(doc)
}
