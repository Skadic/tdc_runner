use serde::{Deserialize, Serialize};
use std::process::{Command, Output};

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let json_file_name = args.next().expect("Please input a tdc json file.");
    let json_str = std::fs::read_to_string(&json_file_name).expect("Could not read file");
    let cfg: AlgorithmConfig =
        serde_json::from_str(json_str.as_str()).expect("Error parsing to json");

    let input_file_name = args.next().expect("Please input a file to compress");

    let algo = &cfg.algo;

    let exe = "./tdc";
    let mut args = vec!["-a".to_owned()];

    args.push(process_algo(algo));

    args.push(input_file_name.clone());

    args.push("-o".to_owned());
    args.push(format!(
        "out/{}_{}.tdc",
        json_file_name.split('/').last().unwrap(),
        input_file_name.split('/').last().unwrap()
    ));

    if cfg.charts {
        args.push("--stats".to_owned());
    }
    if cfg.force {
        args.push("-f".to_owned());
    }
    if cfg.sentinel {
        args.push("-0".to_owned());
    }

    let output = execute(exe, &args.iter().map(AsRef::as_ref).collect::<Vec<_>>())?;
    std::fs::write(
        format!(
            "charts/{}_{}.json",
            json_file_name.split('/').last().unwrap(),
            input_file_name.split('/').last().unwrap()
        ),
        output.stdout.as_slice(),
    )
}

fn process_algo(algo: &Algorithm) -> String {
    let mut s = algo.name.to_owned();
    if let Some(serde_json::Value::Object(ref map)) = algo.data {
        s.push('(');

        s += map
            .into_iter()
            .filter_map(|(key, value)| {
                if let Ok(alg) = serde_json::from_value::<Algorithm>(value.clone()) {
                    Some(format!("{key}={}", process_algo(&alg)))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
            .as_str();
        s.push(')');
    }
    s
}

fn execute(exe: &str, args: &[&str]) -> std::io::Result<Output> {
    Command::new(exe).args(args).output()
}

#[derive(Debug, Serialize, Deserialize)]
struct AlgorithmConfig {
    pub algo: Algorithm,
    pub sentinel: bool,
    pub force: bool,
    pub charts: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Algorithm {
    pub name: String,
    pub data: Option<serde_json::Value>,
}
