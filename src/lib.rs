use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use lazy_static::lazy_static;
use std::sync::Mutex;

struct NameParts {
    praenomen: Vec<&'static str>,
    nomen: Vec<&'static str>,
    cognomen: Vec<&'static str>,
}

pub struct NameConfig {
    pub praenomen: bool,
}

impl NameParts {
    fn new() -> Self {
        let praenomen_data = include_str!("../names/praenomen");
        let nomen_data = include_str!("../names/nomen");
        let cognomen_data = include_str!("../names/cognomen");

        NameParts {
            praenomen: praenomen_data.lines().collect(),
            nomen: nomen_data.lines().collect(),
            cognomen: cognomen_data.lines().collect(),
        }
    }
}

lazy_static! {
    /// A thread-safe HashSet to store generated names.
    static ref GENERATED_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub fn romanname(config: NameConfig) -> Option<String> {
    let name_parts = NameParts::new();
    let mut rng = thread_rng();
    let max_attempts = 1000;

    for _ in 0..max_attempts {
        let mut name = String::new();

        let n = name_parts.nomen.choose(&mut rng).unwrap();
        let c = name_parts.cognomen.choose(&mut rng).unwrap();

        if config.praenomen {
            let p = name_parts.praenomen.choose(&mut rng).unwrap();
            name = format!("{}{}{}{}{}", p, " ", n, " ", c);
        } else {
            name = format!("{}{}{}", n, " ", c);
        }

        let mut generated = GENERATED_NAMES.lock().unwrap();

        if !generated.contains(&name) {
            generated.insert(name.clone());
            return Some(name);
        }
    }

    None
}