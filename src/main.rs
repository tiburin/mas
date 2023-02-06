use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
const OFF: &str = "word.off";
const ON: &str = "word.on";
const MATCH: &str = "";
const MATCHEND: bool = true;
const SORT_BY_POPULAR: bool = false;
const MUST_CONTAINS_WORDS: bool = false;
const SPANISH: bool = true;
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

fn get_spanish(letter: char) -> bool {
    let mut store = HashSet::new();
    "ñáíéóúü".chars().for_each(|n| {
        store.insert(n);
    });
    store.contains(&letter)
}
struct English;
impl English {
    fn start() -> i32 {
        97
    }
    fn end() -> i32 {
        122
    }

    fn is_range(letter: char) -> bool {
        let value = letter as i32;
        let is_english = value >= English::start() && value <= English::end();

        if SPANISH {
            is_english || get_spanish(letter)
        } else {
            is_english
        }
    }
}

fn counting_utf8(input: &str, take: usize, from_end: bool) -> usize {
    if from_end {
        input.chars().rev().take(take).map(|n| n.len_utf8()).sum()
    } else {
        input.chars().take(take).map(|n| n.len_utf8()).sum()
    }
}
struct Str;

impl Str {
    fn rm_start(input: &str) -> &str {
        let mut start = 0;
        let mut list = input.chars();
        let mut current = list.next();
        while let Some(letter) = current {
            if !English::is_range(letter) {
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
            if !English::is_range(letter) {
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
            if !English::is_range(letter) {
                return false;
            }
        }
        input.len() > 0
    }
    pub fn is_ing(input: &str) -> bool {
        if input.len() <= 3 {
            return false;
        }

        let start = input.len() - counting_utf8(input, 3, true);
        &input[start..input.len()] == "ing"
    }
    pub fn is_ed(input: &str) -> bool {
        if input.len() <= 2 {
            return false;
        }
        let start = input.len() - counting_utf8(input, 2, true);
        &input[start..input.len()] == "ed"
    }
    pub fn is_plural(input: &str) -> bool {
        if input.len() <= 2 {
            return false;
        }
        let start = input.len() - counting_utf8(input, 1, true);
        let x = counting_utf8(input, 2, true);
        let a = x - counting_utf8(input, 1, true);

        let prev_start = start - a;
        let letter = &input[start..input.len()];
        letter == "s" && &input[prev_start..start] != "s"
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
    fn sort_popular(list: Vec<&str>) -> Vec<&str> {
        if SORT_BY_POPULAR {
            let store = list.iter().fold(HashMap::new(), |mut store, word| {
                if let Some(word) = store.get_mut(word) {
                    *word += 1;
                } else {
                    store.insert(*word, 1);
                }
                store
            });

            let mut new_list: Vec<_> = store.into_iter().collect();
            new_list.sort_by(|(_, a), (_, b)| a.cmp(b));
            new_list.reverse();
            return new_list.iter().map(|(word, _)| *word).collect();
        }
        list
    }
    pub fn lines(input: &str) -> Vec<String> {
        let mut cache: HashSet<String> = HashSet::new();
        let mut list = Vec::new();
        for word in Parse::sort_popular(input.split_whitespace().collect()) {
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

        list
    }
}

struct Forbid;
impl Forbid {
    fn start(content: &str) -> Tipo {
        let mut store: Tipo = HashSet::new();
        for line in Parse::lines(content) {
            store.insert(line);
        }
        store
    }
}
#[derive(Debug)]
struct Writer {
    path: String,
    len: usize,
    content: String,
}
impl Writer {
    fn new(path: String, content: String, len: usize) -> Self {
        Self { path, len, content }
    }
}

#[derive(Debug)]
struct Voc {
    list: Vec<String>,
    ing: Vec<String>,
    ed: Vec<String>,
    plural: Vec<String>,
    simple: Vec<String>,
    matching: Vec<String>,
    writer: Vec<Option<Writer>>,
}
impl Voc {
    fn new(list: Vec<String>) -> Self {
        Self {
            list,
            ing: vec![],
            ed: vec![],
            plural: vec![],
            simple: vec![],
            matching: vec![],
            writer: Vec::new(),
        }
    }
    fn start(store: &mut Tipo, content: &str) -> Vec<String> {
        let data = Parse::lines(content).into_iter();
        if MUST_CONTAINS_WORDS {
            let mut errors = vec![];
            let on_file_data = data.clone().fold(HashSet::new(), |mut acc, word| {
                acc.insert(word);
                acc
            });

            for word in store.iter() {
                if !on_file_data.contains(word) {
                    errors.push(word);
                }
            }

            for word in &errors {
                println!("({}) does not exist! in word.on", &word);
            }
            if errors.len() > 0 {
                panic!("contains duplicate words in total: {}", errors.len());
            }
        }

        data.filter(|n| !store.contains(n)).collect()
    }
    fn write(name: &str, list: &Vec<String>) -> Option<Writer> {
        let path = if name != ON {
            format!("parts/{}", name)
        } else {
            name.to_owned()
        };
        if list.len() > 0 || name == ON {
            let content = format!("{}\n", list.join("\n"));
            let mi_writter = Writer::new(path, content, list.len());
            return Some(mi_writter);
        }

        None
    }

    fn compose(&mut self) -> &mut Self {
        self.writer.push(Voc::write(ON, &self.list));
        self.writer.push(Voc::write("match.on", &self.matching));

        self.writer.append(&mut Voc::insert(&self.ed, "N"));
        self.writer.append(&mut Voc::insert(&self.ing, "O"));
        self.writer.append(&mut Voc::insert(&self.plural, "P"));
        self.writer.append(&mut Voc::insert(&self.simple, "F"));
        self
    }
    fn direct_data(&mut self) -> &mut Self {
        for word in self.list.clone() {
            if Str::is_match(&word) {
                self.matching.push(word)
            } else if Str::is_ing(&word) {
                self.ing.push(word)
            } else if Str::is_ed(&word) {
                self.ed.push(word)
            } else if Str::is_plural(&word) {
                self.plural.push(word)
            } else {
                self.simple.push(word)
            }
        }

        self
    }
    fn collect(store: &mut HashMap<usize, Vec<String>>, letter: &str) -> Vec<Option<Writer>> {
        let mut inner = vec![];
        for rank in store.keys() {
            let list = store.get(rank).unwrap();
            inner.push(Voc::write(&format!("{}-{}.on", letter, rank), list));
        }
        inner
    }
    fn insert(list: &Vec<String>, letter: &str) -> Vec<Option<Writer>> {
        let mut store = Voc::store();

        for word in list {
            if let Some(node) = store.get_mut(&word.len()) {
                node.push(word.to_owned());
            } else {
                panic!("wrong length invalid data should't be at this point")
            }
        }
        Voc::collect(&mut store, letter)
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
    fn write_to_files(&mut self) {
        for writer in &self.writer {
            if let Some(write) = writer {
                fs::write(&write.path, &write.content).unwrap();
            }
        }
        eprintln!("TOTAL: {}", self.list.len());
    }
}

struct App {
    off_content: String,
    on_content: String,
    store: HashSet<String>,
}

impl App {
    fn new(on_content: String, off_content: String) -> Self {
        Self {
            on_content,
            off_content,
            store: HashSet::new(),
        }
    }
    fn forbid(&mut self) -> &mut Self {
        self.store = Forbid::start(&self.off_content);
        self
    }
    fn start(&mut self) -> Vec<String> {
        Voc::start(&mut self.store, &self.on_content)
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

    let on_content = fs::read_to_string(ON).unwrap();
    let off_content = fs::read_to_string(OFF).unwrap();
    let list = App::new(on_content, off_content).forbid().start();
    Voc::new(list).direct_data().compose().write_to_files();
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
