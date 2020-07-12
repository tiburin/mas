use std::collections::HashSet;
use std::fs;

const OFF: &str = "word.off";
const ON: &str = "word.on";
const PLURAL: &str = "plural.on";
const SIMPLE: &str = "simple.on";
const ED: &str = "ed.on";
const ING: &str = "ing.on";
const MATCH: &str = "ion";
const MATCHEND: bool = true;
type Tipo = HashSet<String>;

mod rule {
    pub struct Word {}
    impl Word {
        pub fn min() -> usize {
            3
        }
        pub fn max() -> usize {
            25
        }
    }
    pub fn is_min(word: &str) -> bool {
        word.len() < Word::min()
    }
    pub fn is_max(word: &str) -> bool {
        word.len() > Word::max()
    }
}
struct English;
impl English {
    fn start() -> i32 {
        97
    }
    fn end() -> i32 {
        122
    }

    fn is_range(value: i32) -> bool {
        value >= English::start() && value <= English::end()
    }
}
struct Str;
impl Str {
    fn rm_start(input: &str) -> &str {
        let mut start = 0;
        let mut list = input.chars();
        let mut current = list.next();
        while let Some(letter) = current {
            if !English::is_range(letter as i32) {
                start += letter.len_utf8();
            } else {
                return &input[start..input.len()];
            }
            current = list.next();
        }
        &input[start..input.len()]
    }
    fn utf_count(input: &str) -> usize {
        input.chars().fold(0, |acc, letter| acc + letter.len_utf8())
    }
    fn rm_end(input: &str) -> &str {
        let mut end = Str::utf_count(input);
        let mut list = input.chars();
        let mut current = list.next_back();
        while let Some(letter) = current {
            if !English::is_range(letter as i32) {
                end -= letter.len_utf8();
            } else {
                return &input[0..end];
            }
            current = list.next_back();
        }
        &input[0..end]
    }
    fn rm_start_end(input: &str) -> &str {
        Str::rm_end(Str::rm_start(input))
    }
    fn valid_english(input: &str) -> bool {
        for letter in input.chars() {
            if !English::is_range(letter as i32) {
                return false;
            }
        }
        true
    }
    pub fn is_ing(input: &str) -> bool {
        let start = input.len() - 3;
        &input[start..input.len()] == "ing"
    }
    pub fn is_ed(input: &str) -> bool {
        let start = input.len() - 2;
        &input[start..input.len()] == "ed"
    }
    pub fn is_plural(input: &str) -> bool {
        let start = input.len() - 1;
        let letter = &input[start..input.len()];
        letter == "s" && &input[start - 1..start] != "s"
    }
    pub fn is_match(input: &str) -> bool {
        if input.len() < 4 || MATCH.trim() == "" {
            return false;
        }
        if MATCHEND {
            &input[input.len() - MATCH.len()..input.len()] == MATCH
        } else {
            &input[0..MATCH.len()] == MATCH
        }
    }
    pub fn get_word(word: &str) -> Option<&str> {
        if rule::is_min(word) || rule::is_max(word) {
            return None;
        }
        Some(word)
    }
}

struct Parse;
impl Parse {
    pub fn lines(input: String) -> Vec<String> {
        let mut cache: HashSet<String> = HashSet::new();
        let mut list = Vec::new();
        for lines in input.split("\n") {
            for word in lines.split_whitespace() {
                let word = word.trim().to_lowercase();
                let word = Str::rm_start_end(&word);
                if let Some(word) = Str::get_word(word) {
                    let contain = cache.contains(word);
                    if !contain && Str::valid_english(word) {
                        list.push(word.to_owned());
                        cache.insert(word.to_owned());
                    }
                }
            }
        }
        list
    }
}

struct Forbid;
impl Forbid {
    fn start() -> Tipo {
        let mut store: Tipo = HashSet::new();
        let content = fs::read_to_string(OFF).unwrap();
        for line in Parse::lines(content) {
            store.insert(line);
        }
        store
    }
}

struct Voc;
impl Voc {
    fn start(store: &mut Tipo) {
        let content = fs::read_to_string(ON).unwrap();
        let list: Vec<String> = Parse::lines(content)
            .into_iter()
            .filter(|n| !store.contains(n))
            .collect();
        Voc::end(list);
    }
    fn write(name: &str, list: Vec<String>) {
        let path = if name != ON {
            format!("parts/{}", name)
        } else {
            name.to_owned()
        };
        fs::write(path, format!("{}\n", list.join("\n"))).unwrap();
    }
    fn end(list: Vec<String>) {
        let size = list.len();
        let mut ing = vec![];
        let mut ed = vec![];
        let mut plural = vec![];
        let mut simple = vec![];
        let mut matching = vec![];
        for word in list.clone() {
            if Str::is_ing(&word) {
                ing.push(word)
            } else if Str::is_ed(&word) {
                ed.push(word)
            } else if Str::is_plural(&word) {
                plural.push(word)
            } else if Str::is_match(&word) {
                matching.push(word)
            } else {
                simple.push(word)
            }
        }

        Voc::write(SIMPLE, simple);
        Voc::write(PLURAL, plural);
        Voc::write(ED, ed);
        Voc::write(ING, ing);
        Voc::write(ON, list);
        Voc::write("match.on", matching);
        eprintln!("TOTAL: {}", size);
    }
}

fn main() {
    for name in vec![ON, OFF] {
        if !std::path::Path::new(name).exists() {
            fs::File::create(name).unwrap();
        }
    }
    if !std::path::Path::new("parts").is_dir() {
        fs::create_dir("parts").unwrap()
    }
    Voc::start(&mut Forbid::start());
}

#[cfg(test)]
mod tests {
    #[test]
    fn start_test() {}
}