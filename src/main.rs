use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    io::ErrorKind,
    iter::once,
    path::PathBuf,
};

use regex::Regex;

fn main() {
    let words = words("big.txt".into());
    let word_map = counter(words);
    //println!("{:?}", map);
    assert_eq!(correction("speling", &word_map), "spelling".to_string());
    println!("{:?}", correction("korrectud", &word_map));
}

fn words(path: PathBuf) -> Vec<String> {
    // add code here
    let content = match read_to_string(path) {
        Ok(c) => c,
        Err(e) if e.kind() == ErrorKind::InvalidData => {
            panic!("{:?}", "Text includes non-utf-8");
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    };
    let re = Regex::new(r"\w+").unwrap();
    let matches = re
        .find_iter(&content)
        .map(|m| m.as_str().to_lowercase())
        .collect::<Vec<_>>();
    matches
}

fn counter(words: Vec<String>) -> HashMap<String, i32> {
    let mut word_map = HashMap::new();
    for word in words {
        word_map.entry(word).and_modify(|n| *n += 1).or_insert(1);
    }
    word_map
}

fn P(word: &str, word_map: &HashMap<String, i32>) -> i32 {
    let num = word_map.values().fold(0, |acc, v| acc + v);
    let freq = word_map.get(word).expect("word not existent");
    *freq / num
}

fn edits1(word: &str) -> HashSet<String> {
    let letters = "abcdefghijklmnopqrstuvwxyz".to_string();
    let splits = splits(word.to_owned());
    let deletes = deletes(&splits);
    let transposes = transposes(&splits);
    let replaces = replaces(&splits, &letters);
    let inserts = inserts(&splits, &letters);
    deletes
        .into_iter()
        .chain(
            transposes
                .into_iter()
                .chain(replaces.into_iter().chain(inserts.into_iter())),
        )
        .collect::<HashSet<_>>()
}

fn edits2(word: &str) -> Vec<String> {
    let mut list = vec![];
    for e1 in edits1(word) {
        for e2 in edits1(&e1) {
            list.push(e2);
        }
    }
    list
}

fn known(words: &[String], map: &HashMap<String, i32>) -> HashSet<String> {
    let words_set = words.iter().cloned().collect::<HashSet<_>>();
    let all_words_set = map.keys().cloned().collect::<HashSet<_>>();
    let set = all_words_set
        .intersection(&words_set)
        .cloned()
        .collect::<HashSet<_>>();
    set
}

fn candidates(word: &str, map: &HashMap<String, i32>) -> HashSet<String> {
    if !known(&once(word.to_string()).collect::<Vec<_>>(), map).is_empty() {
        known(&once(word.to_string()).collect::<Vec<_>>(), map)
    } else if !known(&edits1(word).into_iter().collect::<Vec<_>>(), map).is_empty() {
        known(&edits1(word).into_iter().collect::<Vec<_>>(), map)
    } else if !known(&edits2(word), map).is_empty() {
        known(&edits2(word), map)
    } else {
        let mut set = HashSet::new();
        set.insert(word.to_string());
        set
    }
}

fn correction(word: &str, word_map: &HashMap<String, i32>) -> String {
    let set = candidates(word, &word_map);
    set.into_iter()
        .max_by(|x, y| P(x, &word_map).cmp(&P(y, &word_map)))
        .expect("one mut be the greatest")
}
//Helpers
fn splits(word: String) -> Vec<(String, String)> {
    let mut splits = vec![];
    let len = word.len();
    let range = 0..len + 1;
    for i in range {
        splits.push((word[..i].to_string(), word[i..].to_string()))
    }
    splits
}

fn deletes(splits: &Vec<(String, String)>) -> Vec<String> {
    splits
        .iter()
        .cloned()
        .filter(|(_, r)| !r.is_empty())
        .map(|(mut l, r)| {
            l.push_str(&r[1..]);
            l
        })
        .collect::<Vec<_>>()
}

fn transposes(splits: &Vec<(String, String)>) -> Vec<String> {
    splits
        .iter()
        .cloned()
        .filter(|(_, r)| r.len() > 1)
        .map(|(mut l, r)| {
            let r_regs = r.chars().collect::<Vec<_>>();
            l.push(r_regs[1]);
            l.push(r_regs[0]);
            l.push_str(
                &r_regs[2..]
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .concat(),
            );
            l
        })
        .collect::<Vec<_>>()
}

fn replaces(splits: &Vec<(String, String)>, letters: &String) -> Vec<String> {
    let mut repl = vec![];
    for c in letters.chars() {
        for (l, r) in splits.iter() {
            if !r.is_empty() {
                let mut l = l.clone();
                l.push(c);
                l.push_str(&r[1..].to_string());
                repl.push(l.to_owned());
            }
        }
    }
    repl
}

fn inserts(splits: &Vec<(String, String)>, letters: &String) -> Vec<String> {
    let mut ins = vec![];
    for c in letters.chars() {
        for (l, r) in splits {
            let mut l = l.clone();
            l.push(c);
            l.push_str(r);
            ins.push(l.to_owned());
        }
    }
    ins
}
