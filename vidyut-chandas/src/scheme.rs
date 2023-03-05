//// Goal is to take an slp1 encoded sanskrit verse and extract the
//// G-L pattern from it.

use lazy_static::lazy_static;

type Sound = char;
//hrasva Vowels
const hrasva: &str = "aiufx";
//dIrgha Vowels
const dirgha: &str = "AIUeEoOFX";
//Consonants
const HAL: &str = "kKgGNcCjJYwWqQRtTdDnpPbBmyrlvSzsh"; 
//Anusvara or Visarga
const others: &str = "MH";

#[derive(Debug)]
//// I used enum for learning purposes
/// If it is not useful will change
pub enum Metre {
    G,
    L,
}

//// Returns the enum type converted to char
impl Metre {
    pub fn unwrap(&self) -> char{
        match *self {
            Metre::G => 'G',
            Metre::L => 'L',
        }
    }
}

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

    /// Creates a set whose members are the characters in `string`.
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

////Helper functions to identify the sanskrit sound

pub fn is_hrasva(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(hrasva);
    }
    CHARS.contains(c)
}

pub fn is_dirgha(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(dirgha);
    }
    CHARS.contains(c)
}

pub fn is_HAL(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(HAL);
    }
    CHARS.contains(c)
}

pub fn is_special(c: Sound) -> bool {
    lazy_static! {
        static ref CHARS: Set = Set::from(others);
    }
    CHARS.contains(c)
}


//// Returning a vector of enums that holds the guru-laghu scheme of the input
/// The only reason I used enums is because I was learning them
/// May need to change?
pub fn find_scheme(raw: &String) -> Vec::<Metre> {
    let mut scheme = Vec::<Metre>::new();
    for i in 0..raw.len() {
        let curr: Sound = raw.chars().nth(i).unwrap();
        //Other than from second last char
        if i<=raw.len()-3 {
            let next: Sound = raw.chars().nth(i+1).unwrap();
            let next_next: Sound = raw.chars().nth(i+2).unwrap();
            if is_dirgha(curr) {
                scheme.push(Metre::G);
            } else if is_hrasva(curr) && is_HAL(next) && is_HAL(next_next){
                scheme.push(Metre::G);
            }  else if is_hrasva(curr) && is_special(next){
                scheme.push(Metre::G);
            } else if is_hrasva(curr){
                scheme.push(Metre::L);
            }
        } else if i == raw.len()-2{
            let next: Sound = raw.chars().nth(i+1).unwrap();
            //From second last character to last character it is Laghu only if
            //Dirgha vowel and followed by anusvara/visarga.
            if is_dirgha(curr){
                scheme.push(Metre::G);
            } else if is_hrasva(curr) && is_special(next){
                scheme.push(Metre::G);
            } else if is_hrasva(curr) {
                scheme.push(Metre::L);
            }
        } else {
            if is_dirgha(curr) {
                scheme.push(Metre::G);
            } else if is_hrasva(curr){
                scheme.push(Metre::L);
            }
        }
        
    }

    scheme
}