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

pub fn matches(a: &String, s: &Vec::<Metre>) -> bool {
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
            println!("CHECKING FOR GANTE:");
            println!("{:?},{:?}", found, expected);
            if found == 'L' && expected == 'G' {
                return false;
            }
            if found == 'G' && expected == 'L' {
                gantE = true;
            } 
        }

        //// If gantE is true, print a warning message
        if gantE {
            println!("The last syllable of one or more lines does not match this metre but this is allowed by गन्ते rule!");
        }

        // for i in 0..s.len()/4 {
        //     let found = s[i].unwrap();
        //     let expected = a.chars().nth(i).unwrap();
        //     if s[i].unwrap() != a.chars().nth(i).unwrap() {
        //         return false;
        //     }
        // }

        return true;
    }
}

//// Function that takes the scheme as the input and returns 
//// the name of the metre
//// To do this it reads the JSON file through read_json()

pub fn identify (s: &Vec::<Metre>) -> String {
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

        //Match this vector of strings with the G-L scheme
        if matches( &vec[0] , s ){
            return String::from(metre_name);
        }

    }
    //// If no Pattern found
    String::from("Metre Not Found! Sorry for that :(")
    
}