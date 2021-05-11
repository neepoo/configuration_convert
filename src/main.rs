use std::fs::File;
use std::io::Read;
use std::io;

use serde_yaml::{self, to_value};
use clap::{Arg, App};
use anyhow::Result;
use std::process::exit;

static INPUT_FILE_TYPE: &str = "input file type";
static OUTPUT_FILE_TYPE: &str = "output file type";
static NAME: &str = "configuration file converter";
static INPUT_FILE: &str = "file";


fn yaml_convert_to_json(yaml: serde_yaml::Value) -> serde_json::Value {
    match yaml {
        serde_yaml::Value::Bool(b) => serde_json::Value::Bool(b),
        serde_yaml::Value::Number(number) => {
            if number.is_i64() {
                serde_json::Value::Number(number.as_i64().unwrap().into())
            } else if number.is_u64() {
                serde_json::Value::Number(number.as_u64().unwrap().into())
            } else {
                let n = serde_json::Number::from_f64(number.as_f64().unwrap()).expect("float infinite and nan not allowed");
                serde_json::Value::Number(n)
            }
        }
        serde_yaml::Value::String(s) => serde_json::Value::String(s),
        serde_yaml::Value::Null => serde_json::Value::Null,
        serde_yaml::Value::Sequence(l) => serde_json::Value::Array(l.into_iter().map(yaml_convert_to_json).collect()),
        serde_yaml::Value::Mapping(d) => serde_json::Value::Object(d.into_iter().map(|(k, v)| (k.as_str().unwrap().to_owned(), yaml_convert_to_json(v))).collect())
    }
}

fn toml_convert_to_json(toml: toml::Value) -> serde_json::Value {
    match toml {
        toml::Value::String(s) => serde_json::Value::String(s),
        toml::Value::Integer(i) => serde_json::Value::Number(i.into()),
        toml::Value::Float(f) => {
            let n = serde_json::Number::from_f64(f).expect("float infinite and nan not allowed");
            serde_json::Value::Number(n)
        }
        toml::Value::Boolean(b) => serde_json::Value::Bool(b),
        toml::Value::Array(arr) => serde_json::Value::Array(arr.into_iter().map(toml_convert_to_json).collect()),
        toml::Value::Table(table) => {
            serde_json::Value::Object(table.into_iter().map(|(k, v)| (k, toml_convert_to_json(v))).collect())
        }
        toml::Value::Datetime(dt) => serde_json::Value::String(dt.to_string()),
    }
}

//
//
fn json_to_yaml(json: serde_json::Value) -> serde_yaml::Value {
    to_value(json).unwrap()
}

fn json_to_toml(json: serde_json::Value) -> toml::Value {
    match json {
        serde_json::Value::String(s) => toml::Value::String(s),
        serde_json::Value::Number(i) => {
            if i.is_f64() {
                toml::Value::Float(i.as_f64().unwrap())
            } else if i.is_i64() {
                toml::Value::Integer(i.as_i64().unwrap())
            } else {
                toml::Value::Integer(i.as_i64().unwrap() as i64)
            }
        }
        serde_json::Value::Bool(b) => toml::Value::Boolean(b),
        serde_json::Value::Array(arr) => toml::Value::Array(arr.into_iter().map(json_to_toml).collect()),
        serde_json::Value::Object(obj) => toml::Value::Table(obj.into_iter().map(|(k, v)| (k, json_to_toml(v))).collect()),
        serde_json::Value::Null => toml::Value::String("null".to_string())
    }
}

fn read(filepath: &str) -> io::Result<String> {
    let mut input = String::new();

    if filepath == "-" {
        io::stdin().read_to_string(&mut input)?
    } else {
        File::open(filepath)
            .and_then(|mut f| f.read_to_string(&mut input))?
    };
    Ok(input)
}

pub enum Values {
    Json(serde_json::Value),
    Yaml(serde_yaml::Value),
    Toml(toml::Value),
}


pub fn from_str(s: &str, config_type: ConfigType) -> Result<Values>
{
    match config_type {
        ConfigType::Json => {
            let a = serde_json::from_str(s)?;
            Ok(Values::Json(a))
        }
        ConfigType::Toml => {
            let t = toml::from_str(s)?;
            Ok(Values::Toml(t))
        }
        ConfigType::Yaml => {
            let y = serde_yaml::from_str(s)?;
            Ok(Values::Yaml(y))
        }
    }
}

// 无论src_type, dst_type dou zhuan huan cheng jsn
pub fn other_value_to_json_value(v: Values) -> serde_json::Value {
    match v {
        Values::Json(json) => json,
        Values::Toml(toml) => {
            toml_convert_to_json(toml)
        }
        Values::Yaml(yaml) => {
            yaml_convert_to_json(yaml)
        }
    }
}


pub enum ConfigType {
    Json,
    Yaml,
    Toml,
}

fn main() -> io::Result<()> {
    let matches = App::new(NAME)
        .version("0.0.1")
        .author("wei zhikai <neepoowzk@gmail.com>")
        .about("Different types of configuration file format conversion, support (json, toml, yaml).")
        .arg(Arg::new(INPUT_FILE_TYPE)
            .long("input_file_type")
            .short('s')
            .value_name("src_type")
            .about("input configuration file type(json, toml, yaml)")
            .required(true)
            .takes_value(true))

        .arg(Arg::new(OUTPUT_FILE_TYPE)
            .long("output_file_type")
            .short('d')
            .value_name("dst_type")
            .about("output configuration file type(json, toml, yaml)")
            .required(true)
            .takes_value(true))

        .arg(Arg::new(INPUT_FILE)
            .value_name(INPUT_FILE)
            .takes_value(true))

        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let src_type = matches.value_of(INPUT_FILE_TYPE).unwrap();
    match src_type {
        "json" | "toml" | "yaml" => {}
        _ => {
            eprintln!("input file type only support(json, yaml, toml) type, not support {}!", src_type);
            exit(-1)
        }
    }


    let dst_type = matches.value_of(OUTPUT_FILE_TYPE).unwrap();
    match dst_type {
        "json" | "toml" | "yaml" => {}
        _ => {
            eprintln!("output file type only support(json, yaml, toml) type, not support {}!", dst_type);
            exit(-1)
        }
    }
    let input_file = matches.value_of(INPUT_FILE).unwrap_or("-");
    let input_string = read(input_file)?;
    let src_type_enum = match src_type {
        "json" => ConfigType::Json,
        "toml" => ConfigType::Toml,
        _ => ConfigType::Yaml
    };
    let values = from_str(input_string.as_str(), src_type_enum);
    let middle_json_values = match values {
        Ok(values) => other_value_to_json_value(values),
        Err(e) => {
            let _input_file = match input_file {
                "-" => "stdin",
                filename => filename
            };
            eprintln!("parse input file <{}> error: {}", _input_file, e.to_string());
            exit(-1);
        }
    };
    // let middle_json_values = (values.);


    let dst_type_enum = match dst_type {
        "json" => ConfigType::Json,
        "toml" => ConfigType::Toml,
        _ => ConfigType::Yaml
    };
    match dst_type_enum {
        ConfigType::Json => println!("{}", serde_json::to_string_pretty(&middle_json_values).unwrap()),
        ConfigType::Yaml => {
            let yaml_value = json_to_yaml(middle_json_values);
            println!("{}", serde_yaml::to_string(&yaml_value).unwrap())
        }
        ConfigType::Toml => {
            let toml_value = json_to_toml(middle_json_values);
            println!("{}", toml::to_string(&toml_value).unwrap())
        }
    }
    Ok(())
}

