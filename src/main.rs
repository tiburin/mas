use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
const OFF: &str = "word.off";
const ON: &str = "word.on";
const MATCH: &str = "";
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
        input.len() > 0
    }
    pub fn is_ing(input: &str) -> bool {
        if input.len() <= 3 {
            return false;
        }
        let start = input.len() - 3;
        &input[start..input.len()] == "ing"
    }
    pub fn is_ed(input: &str) -> bool {
        if input.len() <= 2 {
            return false;
        }
        let start = input.len() - 2;
        &input[start..input.len()] == "ed"
    }
    pub fn is_plural(input: &str) -> bool {
        if input.len() <= 2 {
            return false;
        }
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
    fn utf_count(input: &str) -> usize {
        input.chars().fold(0, |acc, letter| acc + letter.len_utf8())
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
        if list.len() > 0 {
            fs::write(path, format!("{}\n", list.join("\n"))).unwrap();
        }
    }
    fn end(list: Vec<String>) {
        let size = list.len();
        let mut ing = vec![];
        let mut ed = vec![];
        let mut plural = vec![];
        let mut simple = vec![];
        let mut matching = vec![];
        for word in list.clone() {
            if Str::is_match(&word) {
                matching.push(word)
            } else if Str::is_ing(&word) {
                ing.push(word)
            } else if Str::is_ed(&word) {
                ed.push(word)
            } else if Str::is_plural(&word) {
                plural.push(word)
            } else {
                simple.push(word)
            }
        }

        Voc::write(ON, list);
        Voc::write("match.on", matching);
        Voc::next_level(ed, "N");
        Voc::next_level(ing, "O");
        Voc::next_level(plural, "P");
        Voc::next_level(simple, "F");
        eprintln!("TOTAL: {}", size);
    }
    fn process_next_level(store: &mut HashMap<usize, Vec<String>>, letter: &str) {
        for rank in store.keys() {
            let list = store.get(rank).unwrap();
            Voc::write(&format!("{}-{}.on", letter, rank), list.clone())
        }
    }
    fn next_level(list: Vec<String>, letter: &str) {
        let mut store = Voc::store();

        for word in list {
            match word.len() {
                3 => store.get_mut(&3).unwrap().push(word),
                4 => store.get_mut(&4).unwrap().push(word),
                5 => store.get_mut(&5).unwrap().push(word),
                6 => store.get_mut(&6).unwrap().push(word),
                7 => store.get_mut(&7).unwrap().push(word),
                8 => store.get_mut(&8).unwrap().push(word),
                9 => store.get_mut(&9).unwrap().push(word),
                10 => store.get_mut(&10).unwrap().push(word),
                11 => store.get_mut(&11).unwrap().push(word),
                12 => store.get_mut(&12).unwrap().push(word),
                13 => store.get_mut(&13).unwrap().push(word),
                14 => store.get_mut(&14).unwrap().push(word),
                15 => store.get_mut(&15).unwrap().push(word),
                16 => store.get_mut(&16).unwrap().push(word),
                17 => store.get_mut(&17).unwrap().push(word),
                18 => store.get_mut(&18).unwrap().push(word),
                19 => store.get_mut(&19).unwrap().push(word),
                20 => store.get_mut(&20).unwrap().push(word),
                21 => store.get_mut(&21).unwrap().push(word),
                22 => store.get_mut(&22).unwrap().push(word),
                23 => store.get_mut(&23).unwrap().push(word),
                24 => store.get_mut(&24).unwrap().push(word),
                25 => store.get_mut(&25).unwrap().push(word),
                _ => panic!("wrong length invalid data should't be at this point"),
            }
        }
        Voc::process_next_level(&mut store, letter);
    }
    fn store() -> HashMap<usize, Vec<String>> {
        let mut store = HashMap::new();
        let mut start = rule::Word::min();
        while start <= rule::Word::max() {
            store.insert(start, vec![]);
            start += 1;
        }
        store
    }
}

fn main() {
    for name in vec![ON, OFF] {
        if !std::path::Path::new(name).exists() {
            fs::File::create(name).unwrap();
        }
    }
    if std::path::Path::new("parts").is_dir() {
        fs::remove_dir_all("parts").unwrap();
    }
    fs::create_dir("parts").unwrap();
    Voc::start(&mut Forbid::start());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rule::Word;
    #[test]
    fn store_test() {
        let store = Voc::store();
        for rank in Word::min()..Word::max() {
            assert_eq!(store.get(&rank), Some(&vec![]));
        }
    }
    #[test]
    fn get_word_test() {
        let max_letter = "a".repeat(Word::max() + 1);
        let min_letter = "a".repeat(Word::min() - 1);
        assert!(Str::get_word(&max_letter).is_none());
        assert!(Str::get_word(&min_letter).is_none());
        assert_eq!(Str::get_word("hello"), Some("hello"));
    }
    #[test]
    fn is_plural_test() {
        assert!(!Str::is_plural("es"));
        assert!(!Str::is_plural("discuss"));
        assert!(Str::is_plural("houses"));
    }
    #[test]
    fn is_ed_test() {
        assert!(Str::is_ed("worked"));
        assert!(!Str::is_ed("ed"));
    }
    #[test]
    fn is_ing_test() {
        assert!(Str::is_ing("working"));
        assert!(!Str::is_ing("worknng"));
        assert!(!Str::is_ing(""));
    }
    #[test]
    fn valid_english_test() {
        assert!(Str::valid_english("z"));
        assert!(Str::valid_english("ab"));
        assert!(!Str::valid_english(""));
    }
    #[test]
    fn rm_start_end_test() {
        assert_eq!(Str::rm_start_end("  "), "");
        assert_eq!(Str::rm_start_end(" hello "), "hello");
        assert_eq!(Str::rm_start_end("1/#*hello1/#*"), "hello");
    }
    #[test]
    fn rm_start_test() {
        assert_eq!(Str::rm_start("  "), "");
        assert_eq!(Str::rm_start(" hello"), "hello");
        assert_eq!(Str::rm_start("1/#*hello"), "hello");
    }
    #[test]
    fn rm_end_test() {
        assert_eq!(Str::rm_end("  "), "");
        assert_eq!(Str::rm_end("hello "), "hello");
        assert_eq!(Str::rm_end("hello1/#*"), "hello");
    }
}
