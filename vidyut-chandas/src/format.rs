// Goal is to take a slp1 encoded verse, it's identified metre, and add
// '\n' at pada ends and also to display the scheme of the metre in G-L-Y format (From the database)

use crate::scheme;
use lazy_static::lazy_static;

type Sound = char;
//hrasva Vowels
const HRASVA: &str = "aiufx";
//dIrgha Vowels
const DIRGHA: &str = "AIUeEoOFX";
//Anusvara or Visarga
const OTHERS: &str = "MH";

#[derive(Debug)]
#[allow(dead_code)]
/// A set of Sanskrit sounds.
///
/// This implementation is copied directly from `vidyut_prakriya::sounds`. For details, see the
/// comments there.
pub struct Set([u8; 256]);

impl Set {
    /// Creates an empty set.
    pub fn new() -> Self {
        Set([0; 256])
    }

    /// Creates a set whose members are the Soundacters in `string`.
    pub fn from(string: impl AsRef<str>) -> Self {
        let mut res = Self::new();
        for c in string.as_ref().chars() {
            res.0[c as usize] = 1;
        }
        res
    }

    /// Returns whether the set contains the given sound.
    pub fn contains(&self, c: Sound) -> bool {
        self.0[c as usize] == 1
    }
}

/// Helper Functions to identify the sound

pub fn is_hrasva(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(HRASVA);
    }
    CHARS.contains(c)
}

pub fn is_dirgha(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(DIRGHA);
    }
    CHARS.contains(c)
}

pub fn is_special(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(OTHERS);
    }
    CHARS.contains(c)
}

//Format the verse output such that each syllable starts with consonants and ends with a vowel. Also accounts for H and M
pub fn format_verse(verse: &String, s: &Vec<scheme::Metre>) -> String {
    let mut res: String = String::new();
    let mut i: usize = 0;
    let mut count: usize = 0;
    while i < verse.len() - 1 {
        let c: char = verse.chars().nth(i).unwrap();
        // print!("{}", c);
        res.push(c);
        if is_dirgha(c) || is_hrasva(c) {
            if !is_special(verse.chars().nth(i + 1).unwrap()) {
                // print!(" ");
                res.push_str(&" ");
                count += 1;
                if count == s.len() / 2 || count == s.len() / 4 || count == 3 * s.len() / 4 {
                    // println!();
                    res.push('\n');
                }
            } else {
                // print!("{} ", verse.chars().nth(i+1).unwrap());
                res.push(verse.chars().nth(i + 1).unwrap());
                res.push(' ');
                count += 1;
                if count == s.len() / 2 || count == s.len() / 4 || count == 3 * s.len() / 4 {
                    // println!();
                    res.push('\n');
                }
                i += 1;
            }
        }
        i += 1;
    }
    res.push(verse.chars().nth(verse.len() - 1).unwrap());
    res.push('\n');

    res
}

// Print the scheme of the verse

pub fn print_scheme(s: &Vec<scheme::Metre>) {
    for i in 0..s.len() {
        print!("{:?}", s[i]);
        if i == s.len() / 2 - 1 || i == s.len() / 4 - 1 || i == 3 * s.len() / 4 - 1 {
            println!();
        } else if i != s.len() - 1 {
            print!(" - ");
        }
    }
}
