//// This code has two components.
/// 1) Read the JSON data from https://github.com/shreevatsa/sanskrit/blob/master/data/mishra.json (saved locally)
///    and store it as a rust object
/// 2) Take the scheme of the input verse as the input and
///     check against the data for the metre
use serde::Deserialize;
use serde_json;
use std::fs;

extern crate levenshtein;
use crate::scheme::Metre;
use levenshtein::levenshtein;

//// Structs that store the metrical data from mishra.json

#[derive(Deserialize)]
pub struct VrttaData {
    comment: Vec<String>,
    metres: Vec<Vrtta>,
}

#[derive(Clone, Deserialize)]
struct Vrtta {
    name: String,
    pattern: StringOrList,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum StringOrList {
    String(String),
    List(Vec<String>),
}

//// Function that reads the json data, stores it in an object and
/// returns that object

pub fn read_json() -> VrttaData {
    let path = "./src/data/mishra.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let my_data: VrttaData = serde_json::from_str(&data).expect("Unable to parse");

    //// Just to get rid of error message I am using my_data.comment
    //// I am throwing away the value
    let _ = my_data.comment;

    return my_data;
}

// pub fn check_anushtubh(s: &Vec<Metre>) -> bool {
//     if s.len()
// }

//// Function that matches the scheme against some pattern from the data
/// For now it only matches the first pada of the scheme with
/// the first pattern string. So in effect it works only when all 4 padas
/// have same pattern and गन्ते is ignored.

pub fn identify_sama_vrtta(a: &String, s: &Vec<Metre>) -> usize {
    let mut input_scheme: String = String::new();
    let mut pattern_scheme: String = String::new();
    if a.len() != s.len() / 4 {
        for i in 0..4 {
            _ = i;
            input_scheme += &a;
        }
        for i in 0..s.len() {
            pattern_scheme += &s[i].unwrap().to_string();
        }

        levenshtein(&input_scheme, &pattern_scheme)
    } else {
        let mut gante = false;
        let pada_len = s.len() / 4;
        //// Check except for the last syllable
        for i in 0..s.len() {
            if (i + 1) % pada_len != 0 {
                // println!("{:?}", i);
                let found = s[i].unwrap();
                let expected = a.chars().nth(i % pada_len).unwrap();
                input_scheme += &found.to_string();
                pattern_scheme += &expected.to_string();
            }
        }
        let distance = levenshtein(&input_scheme, &pattern_scheme);
        if distance > 0 {
            return distance;
        }
        //// Now check if last syllable is laghu when it should be guru
        for i in 0..4 {
            let found = s[i * pada_len + pada_len - 1].unwrap();
            let expected = a.chars().last().unwrap();
            println!("CHECKING LAST SYLLABLE:");
            println!(
                "{:?},{:?}, {}",
                found,
                expected,
                match found == expected {
                    true => String::from("SAME"),
                    false => String::from("DIFFERENT"),
                }
            );

            if found != expected {
                gante = true;
            }
        }

        //// If gante is true, print a warning message
        if gante {
            println!("\n\nThe last syllable of one or more lines does not match this metre but some grammatical rules allow this!\n\n");
        }

        0
    }
}

pub fn identify_ardha_sama_vrtta(a: &Vec<String>, s: &Vec<Metre>) -> usize {
    //// TODO:
    //// Find algorithm to find if pattern if an Ardha Sama Vrtta
    //// Two cases arise:
    //// 1) a.len() = 2
    ////     Here we check if 1st, 3rd padas of input s are equal to a[0] and a[1] and
    ////     if 2nd and 4rd padas of input s are equal to a[0] and a[1]
    //// 2) a.len() = 4
    ////     Here we check if 1st, 2nd, 3rd, 4th padas of input s are equal to a[0], a[1], a[2], a[3]
    let mut input_scheme: String = String::new();
    let mut pattern_scheme: String = String::new();
    let distance: usize;
    match a.len() {
        2 => {
            if 2 * a[0].len() + 2 * a[1].len() != s.len() {
                pattern_scheme += &a[0].to_string();
                pattern_scheme += &a[1].to_string();
                pattern_scheme += &a[0].to_string();
                pattern_scheme += &a[1].to_string();
                for t in 0..s.len() {
                    input_scheme += &s[t].unwrap().to_string();
                }
                distance = levenshtein(&pattern_scheme, &input_scheme);
                return distance;
            }
            //// Checking 1st, 3rd padas except for last syllable
            for i in 0..a[0].len() - 1 {
                input_scheme += &s[i].unwrap().to_string();

                if a[0].chars().nth(i).unwrap() != 'X' {
                    pattern_scheme += &a[0].chars().nth(i).unwrap().to_string();
                } else {
                    pattern_scheme += &s[i].unwrap().to_string();
                }
                input_scheme += &s[i + a[0].len() + a[1].len()].unwrap().to_string();
                if a[0].chars().nth(i).unwrap() != 'X' {
                    pattern_scheme += &a[0].chars().nth(i).unwrap().to_string();
                } else {
                    pattern_scheme += &s[i + a[0].len() + a[1].len()].unwrap().to_string();
                }
            }
            //// Checking 2nd, 4th padas except for last syllable
            for i in 0..a[1].len() - 1 {
                input_scheme += &s[i + a[0].len()].unwrap().to_string();
                if a[1].chars().nth(i).unwrap() != 'X' {
                    pattern_scheme += &a[1].chars().nth(i).unwrap().to_string();
                } else {
                    pattern_scheme += &s[i + a[0].len()].unwrap().to_string();
                }

                input_scheme += &s[i + 2 * a[0].len() + a[1].len()].unwrap().to_string();
                if a[1].chars().nth(i).unwrap() != 'X' {
                    pattern_scheme += &a[1].chars().nth(i).unwrap().to_string();
                } else {
                    pattern_scheme += &s[i + 2 * a[0].len() + a[1].len()].unwrap().to_string();
                }
            }
            distance = levenshtein(&input_scheme, &pattern_scheme);
            return distance;
        }
        4 => {
            if a[0].len() + a[1].len() + a[2].len() + a[3].len() != s.len() {
                pattern_scheme += &a[0].to_string();
                pattern_scheme += &a[1].to_string();
                pattern_scheme += &a[2].to_string();
                pattern_scheme += &a[3].to_string();
                for t in 0..s.len() {
                    input_scheme += &s[t].unwrap().to_string();
                }
                for t in 0..pattern_scheme.len() {
                    if pattern_scheme.chars().nth(t).unwrap() == 'X' {
                        if t < s.len() {
                            pattern_scheme.replace_range(
                                t..t + 1,
                                &input_scheme.chars().nth(t).unwrap().to_string(),
                            );
                        } else {
                            pattern_scheme.replace_range(t..t + 1, "G");
                        }
                    }
                }
                distance = levenshtein(&pattern_scheme, &input_scheme);
                return distance;
            }
            for i in 0..a[0].len() - 1 {
                input_scheme += &s[i].unwrap().to_string();
                if a[0].chars().nth(i).unwrap() != 'X' {
                    pattern_scheme += &a[0].chars().nth(i).unwrap().to_string();
                } else {
                    pattern_scheme += &s[i].unwrap().to_string();
                }
            }
            for i in 0..a[1].len() - 1 {
                input_scheme += &s[i + a[0].len()].unwrap().to_string();
                if a[1].chars().nth(i).unwrap() != 'X' {
                    pattern_scheme += &a[1].chars().nth(i).unwrap().to_string();
                } else {
                    pattern_scheme += &s[i + a[0].len()].unwrap().to_string();
                }
            }
            for i in 0..a[2].len() - 1 {
                input_scheme += &s[i + a[0].len() + a[1].len()].unwrap().to_string();
                if a[2].chars().nth(i).unwrap() != 'X' {
                    pattern_scheme += &a[2].chars().nth(i).unwrap().to_string();
                } else {
                    pattern_scheme += &s[i + a[0].len() + a[1].len()].unwrap().to_string();
                }
            }
            for i in 0..a[3].len() - 1 {
                input_scheme += &s[i + a[0].len() + a[1].len() + a[2].len()]
                    .unwrap()
                    .to_string();
                if a[3].chars().nth(i).unwrap() != 'X' {
                    pattern_scheme += &a[3].chars().nth(i).unwrap().to_string();
                } else {
                    pattern_scheme += &s[i + a[0].len() + a[1].len() + a[2].len()]
                        .unwrap()
                        .to_string();
                }
            }
            distance = levenshtein(&input_scheme, &pattern_scheme);
            return distance;
        }
        _ => {
            println!("Some Error Occured! {:?}", a);
        }
    }
    println!("SOME ERROR OCCURED! :(");
    return 99999;
}

pub fn is_sama_vrtta(s: &Vec<Metre>) -> bool {
    let pada_len = s.len() / 4;
    for i in 1..4 {
        for j in 0..pada_len - 1 {
            if s[j].unwrap() != s[i * pada_len + j].unwrap() {
                return false;
            }
        }
    }
    return true;
}

//// Function that takes the scheme as the input and returns
//// the name of the metre
//// To do this it reads the JSON file through read_json()

pub fn identify(s: &Vec<Metre>) -> String {
    if is_sama_vrtta(s) {
        println!("Input is a sama vRtta Metre!");
    }

    let vrtta_kosha: VrttaData = read_json();
    let mut min_distance = 99999;
    let mut closest_metre_name = String::new();
    let mut scheme_of_closest_pattern_ardha = Vec::new();
    let mut scheme_of_closest_pattern_sama = Vec::new();
    let mut closest_is_sama_vrtta = true;
    for i in 0..vrtta_kosha.metres.len() {
        let ref metre_name = vrtta_kosha.metres[i].name;
        //// Find each pattern as a vector of strings.
        //// Right now it is being stored as either
        //// 1) String or 2) List of Strings

        let mut vec = Vec::new();

        match &vrtta_kosha.metres[i].pattern {
            StringOrList::String(a) => {
                vec.push(String::from(a));
            }
            StringOrList::List(b) => {
                vec = b.to_vec();
            }
        }
        let padas = vec.len();
        //// Check even if it is not a sama vrtta because user can make mistake while writing input
        if padas == 1 {
            let tmp = identify_sama_vrtta(&vec[0], s);
            if tmp < min_distance {
                min_distance = tmp;
                closest_metre_name = String::from(metre_name.to_string());
                scheme_of_closest_pattern_sama.push(vec.clone());
                println!("The scheme is: ");
                println!("{:?}", scheme_of_closest_pattern_sama);
                closest_is_sama_vrtta = true;
            }
            if min_distance == 0 {
                println!("The scheme is: ");
                println!("{:?}", scheme_of_closest_pattern_sama);
                return String::from(metre_name.to_string());
            }
        }
        // Check for ardha sama vrtta
        if padas > 1 {
            let tmp = identify_ardha_sama_vrtta(&vec, s);
            if tmp < min_distance {
                min_distance = tmp;
                closest_metre_name = String::from(metre_name.to_string());
                scheme_of_closest_pattern_ardha.push(vec);
                closest_is_sama_vrtta = false;
            }
            if min_distance == 0 {
                if closest_metre_name != "anushtubh" {
                    println!("Input is an ardha-sama-vrtta.");
                }
                println!("The scheme is: ");
                println!("{:?}", scheme_of_closest_pattern_ardha);
                return String::from(closest_metre_name);
            }
        }
    }

    //// If no Pattern found
    println!("A similar metre is: {:?}", closest_metre_name);
    println!(
        "It's levenschtein distance from the input is: {:?}",
        min_distance
    );
    println!("The scheme is: ");
    if closest_is_sama_vrtta {
        println!("{:?}", scheme_of_closest_pattern_sama);
    } else {
        println!("{:?}", scheme_of_closest_pattern_ardha);
    }

    String::from("Metre Not Found! Sorry for that :(")
}
