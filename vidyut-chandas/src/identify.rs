//// This code has two components.
/// 1) Read the JSON data from https://github.com/shreevatsa/sanskrit/blob/master/data/mishra.json (saved locally)
///    and store it as a rust object
/// 2) Take the scheme of the input verse as the input and 
///     check against the data for the metre

use serde::Deserialize;
use serde_json;
use std::fs;

use crate::scheme::Metre;

//// Structs that store the metrical data from mishra.json

#[derive(Debug, Deserialize)]
pub struct vrtta_data {
    
    comment: Vec<String>,
    metres: Vec<vrtta>,
}

#[derive(Clone, Debug, Deserialize)]
struct vrtta {
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

pub fn read_json () -> vrtta_data {
    let path = "./src/mishra.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let my_data: vrtta_data = serde_json::from_str(&data).expect("Unable to parse");
    //println!("{:#?}", my_data);
    return my_data;
}

//// Function that matches the scheme against some pattern from the data
/// For now it only matches the first pada of the scheme with
/// the first pattern string. So in effect it works only when all 4 padas
/// have same pattern and गन्ते is ignored.

pub fn identify_Sama_Vrtta(a: &String, s: &Vec::<Metre>) -> bool {
    //// hardcoded to take only first pada (if all have equal length)
    //// no flexibility for गन्ते also
    if a.len() != s.len()/4 {
        false
    } else {
        let mut gantE = false;
        let pada_len = s.len()/4;
        //// Check except for the last syllable
        for i in 0..s.len() {
            // println!("{:?}", i);
            let found = s[i].unwrap();
            let expected = a.chars().nth(i%pada_len).unwrap();
            if (i+1)%pada_len != 0 {
                if found!=expected {
                    return false;
                }
                // println!("{:?},{:?},{:?}",
                //     found,
                //     expected,
                //     found == expected
                // );
            } else {
                // println!();
            }
        }
        //// Now check if last syllable is laghu when it should be guru
        for i in 0..4 {
            let found = s[i*pada_len+pada_len-1].unwrap();
            let expected = a.chars().last().unwrap();
            println!("CHECKING LAST SYLLABLE:");
            println!("{:?},{:?}, {}", found, expected, 
            
                match found==expected {
                    true => String::from("SAME"),
                    false => String::from("DIFFERENT"),
                    _ => String::from("SOME ERROR HAPPENED!")
                }
        
            );
            
            if found != expected {
                gantE = true;
            }
        }

        //// If gantE is true, print a warning message
        if gantE {
            println!("\n\nThe last syllable of one or more lines does not match this metre but some grammatical rules allow this!\n\n");
        }

        return true;
    }
}

pub fn identify_Ardha_Sama_Vrtta(a: &Vec::<String>, s: &Vec::<Metre>) -> bool {
    //// TODO:
    //// Find algorithm to find if pattern if an Ardha Sama Vrtta
    //// Two cases arise: 
    //// 1) a.len() = 2 
    ////     Here we check if 1st, 3rd padas of input s are equal to a[0] and a[1] and
    ////     if 2nd and 4rd padas of input s are equal to a[0] and a[1]
    //// 2) a.len() = 4
    ////     Here we check if 1st, 2nd, 3rd, 4th padas of input s are equal to a[0], a[1], a[2], a[3]
    
    match a.len(){
        2 => {
            //// Check whether the lengths of input metre and data metre is same or not.
            //// If not same return false
            if 2*a[0].len()+2*a[1].len() != s.len(){
                return false;
            }
            //// Checking 1st, 3rd padas except for last syllable
            for i in 0..a[0].len()-1 {
                if a[0].chars().nth(i).unwrap() != s[i].unwrap() {
                    return false;
                }
                if a[0].chars().nth(i).unwrap() != s[i + a[0].len()+a[1].len()].unwrap() {
                    return false;
                }
            }
            //// Checking 2nd, 4th padas except for last syllable
            for i in 0..a[1].len()-1 {
                if a[1].chars().nth(i).unwrap() != s[i+a[0].len()].unwrap() {
                    return false;
                }
                if a[1].chars().nth(i).unwrap() != s[i + 2*a[0].len()+a[1].len()].unwrap() {
                    return false;
                }
            }
            return true;
        },
        4 => {
            //// Check whether the lengths of input metre and data metre is same or not.
            //// If not same return false
            if a[0].len()+a[1].len()+a[2].len()+a[3].len() != s.len(){
                return false;
            }

            for i in 0..a[0].len(){
                if a[0].chars().nth(i).unwrap() != s[i].unwrap() {
                    return false;
                }
            }
            for i in 0..a[1].len(){
                if a[1].chars().nth(i).unwrap() != s[i + a[0].len()].unwrap() {
                    return false;
                }
            }
            for i in 0..a[2].len(){
                if a[2].chars().nth(i).unwrap() != s[i + a[0].len()+a[1].len()].unwrap() {
                    return false;
                }
            }
            for i in 0..a[3].len(){
                if a[3].chars().nth(i).unwrap() != s[i + a[0].len()+a[1].len()+a[2].len()].unwrap() {
                    return false;
                }
            }
            return true;
        },
        _ => {
            // println!("Some Error Occured! {:?}", a);
        }
    }
    return false;
}

pub fn is_Sama_Vrtta(s: &Vec::<Metre>) -> bool {
    let pada_len = s.len()/4;
    for i in 1..4 {
        for j in 0..pada_len-1 {
            if s[j].unwrap() != s[i*pada_len+j].unwrap() {
                return false;
            }
        }
    }
    return true;
}

//// Function that takes the scheme as the input and returns 
//// the name of the metre
//// To do this it reads the JSON file through read_json()

pub fn identify (s: &Vec::<Metre>) -> String {

    if is_Sama_Vrtta(s){
        println!("Input is a sama vRtta Metre!");
    }

    let vrtta_kosha: vrtta_data = read_json();
    for i in 0..vrtta_kosha.metres.len(){
        let ref metre_name = vrtta_kosha.metres[i].name;
        //// Find each pattern as a vector of strings. 
        //// Right now it is being stored as either 
        //// 1) String or 2) List of Strings
        let mut vec = Vec::new();
        match &vrtta_kosha.metres[i].pattern {
            
            StringOrList::String(a) => {
                vec.push(String::from(a));
            },
            StringOrList::List(b) => {
                vec = b.to_vec();
            },
        }

        //// If the input verse is a Sama_Vrtta, then check against pattern only if 
        //// the pattern is also a Sama_Vrtta
        
        if is_Sama_Vrtta(s){
            if vec.len() > 1 {
                continue
            } else {
                //Match this vector of strings with the G-L scheme
                if identify_Sama_Vrtta( &vec[0] , s ){
                    println!("The input metre is a sama-vrtta.....");
                    println!("{:?}", vec);
                    return String::from(metre_name);
                }
            }
        } else {
            // println!("Checking for ardha sama vrtta");
            if identify_Ardha_Sama_Vrtta(&vec, s){
                println!("The input metre is an ardha-sama-vrtta.....");
                println!("{:?}", vec);
                return String::from(metre_name);
            }
        }

    }
    //// If no Pattern found
    String::from("Metre Not Found! Sorry for that :(")
    
}