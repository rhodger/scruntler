use serde_json::{json, Value, Error};
use rand::seq::SliceRandom;

const API_KEY: &str = "6bd06c2e-4141-4ebc-a8ae-a5221f45ecca";

pub struct Scrunt {
    sentence: String,
    scrunted_sentence: Option<String>,
    pattern: Vec<bool>,
}

pub struct Game {
    player_name: String,
    scrunts: Vec<Scrunt>,
}

impl Game {
    pub fn new(name: &str) -> Self {
        Game {
            player_name: name.to_string(),
            scrunts: Vec::<Scrunt>::new(),
        }
    }

    pub fn add_random(&mut self) {
        let sentence = "im not talking to a worm sucking idiot";
        let mut pattern = Vec::<bool>::new();
        for i in &[false, false, true, false, false, true, true, true] {
            pattern.push(*i);
        }

        self.scrunts.push(Scrunt::new(sentence, pattern));
    }

    pub fn get_sentence(&mut self) -> (String, String) {
        let mut scrunt = self.scrunts.pop().unwrap();
        let clue = scrunt.get_scrunted();
        let solution = scrunt.get_clean();

        return (solution, clue);
    }
}

impl Scrunt {
    fn new(sentence: &str, pattern: Vec<bool>) -> Self {
        Scrunt {
            sentence: sentence.to_string(),
            scrunted_sentence: None,
            pattern: pattern,
        }
    }

    fn scrunt(&mut self) -> Result<(), &'static str> {
        let mut output = String::from("");

        let original = self.sentence.to_string();
        let split = original.split(" ").collect::<Vec<&str>>();
        if split.len() != self.pattern.len() {
            return Err("Failed due to incorrect pattern length");
        } else {
            for i in 0..split.len() {
                if *self.pattern.get(i).unwrap() {
                    if let Ok(alts) = get_synonyms(split[i]) {
                        output = format!("{} {}", output, alts.choose(&mut rand::thread_rng()).expect("Something wrong with the randomness"));
                    } else {
                        return Err("Could not find synonyms for a word");
                    }
                } else {
                    output = format!("{} {}", output, split[i]);
                }
            }
        }

        self.scrunted_sentence = Some(output.to_string());

        return Ok(());
    }

    pub fn get_scrunted(&mut self) -> String {
        if self.scrunted_sentence.is_some() {
            return self.scrunted_sentence.as_ref().unwrap().to_string();
        } else {
            let scrunted = self.scrunt();
            if scrunted.is_ok() {
                return self.get_scrunted();
            } else {
                panic!("Scrunting proved impossible");
            }
        }
    }

    pub fn get_clean(&self) -> String {
        self.sentence.to_string()
    }
}

fn get_synonyms(word: &str) -> Result<Vec<String>, serde_json::Error> {
    let mut syn_list = Vec::<String>::new();
    let request: String = format!(
        "https://www.dictionaryapi.com/\
        api/v3/references/thesaurus/json/{}?key={}", word, API_KEY);

    let data = reqwest::blocking::get(&request).unwrap()
        .text().unwrap();

    // Parse the string of data into serde_json::Value.
    let v: Value = match serde_json::from_str(&data) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    let mut syns = Vec::<&Value>::new();
    // Convert synonyms into iterable vector
    let temp_syns = v[0]["meta"]["syns"][0].as_array();
    if temp_syns.is_some() {
        for i in temp_syns.unwrap() {
            syns.push(i);
        }
    }

    for i in syns {
        let stripped = strip_characters(&i.to_string(), r#"""#);
        syn_list.push(stripped);
    }

    Ok(syn_list)
}

fn strip_characters(original : &str, to_strip : &str) -> String {
    let mut result = String::new();
    for c in original.chars() {
        if !to_strip.contains(c) {
           result.push(c);
       }
    }
    result
}
