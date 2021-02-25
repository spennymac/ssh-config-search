#![feature(str_split_once)]

use std::fs::File;
use std::{io, env};
use std::io::BufRead;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    host: String,
    host_name: String,
    config: std::collections::HashMap<String,String>
}
impl Hash for Entry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.host.hash(state);
    }
}

const SSH_CONFIG: &str = ".ssh/config";

fn main() {
    let home_dir = dirs::home_dir().expect("could not get $HOME dir");

    let config = home_dir.join(SSH_CONFIG);
    let file = File::open(config).expect("could not open ssh config");
    let lines =io::BufReader::new(file).lines();
    let mut entries = std::collections::HashMap::new();

    let mut raw = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#'){
                continue;
            }
            if line.starts_with("Host ") {
                if !raw.is_empty() {
                    let entry = build_entry(&raw);
                    entries.insert(entry.host.clone(), entry);
                    raw.clear();
                }
            }

            raw.push(line.to_owned())
        }
    }
    let args = env::args();
    let patterns = args.into_iter().skip(1).collect::<Vec<String>>();
    let mut contains: HashSet<&Entry> = HashSet::new();

    for p in patterns {
        let c = entries.iter().filter(|(k,_)| {
            k.to_lowercase().contains(&p.to_lowercase())
        }).map(|e| e.1);
        contains.extend(c);
    }

    let mut contains = contains.into_iter().collect::<Vec<&Entry>>();

    contains.sort_by(|a,b| a.host.cmp(&b.host));

    for c in contains {
        println!("{}", c.host)
     }

}
fn build_entry(raw : &[String])  -> Entry {
    let mut host = Default::default();
    let mut hostname= Default::default();
    let mut config = std::collections::HashMap::new();
    for l in raw {
        let (key, value) =  split(l);
        if key == "Host" {
            host = value;
        } else if key == "Hostname" {
                hostname = value;
        } else {
            config.insert(key.to_owned(), value.to_owned());
        }
    }

    if hostname.is_empty() {
        hostname = host;
    }
    Entry {
        host: host.to_owned(),
        host_name: hostname.to_owned(),
        config: config
    }
}

fn split(l : &str)  -> (&str, &str) {
    let white_space = l.find(char::is_whitespace);
    let equals = l.find('=');

    let use_white = white_space.is_some();
    let use_equals = equals.is_some();

    if use_white && !use_equals {
        return l.split_once(char::is_whitespace).unwrap()
    }

    if use_equals && !use_white {
        return l.split_once('=').unwrap()
    }

    let white_space = white_space.unwrap();
    let equals = equals.unwrap();
    if white_space < equals {
        l.split_once(char::is_whitespace).unwrap()
    } else {
        l.split_once('=').unwrap()
    }
}