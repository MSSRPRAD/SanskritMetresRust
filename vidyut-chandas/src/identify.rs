//// This code has two components.
/// 1) Read the JSON data from https://github.com/shreevatsa/sanskrit/blob/master/data/mishra.json
///    and store it as a rust object
/// 2) Take the scheme of the input verse as the input and 
///     check against the data for the metre

use serde::Deserialize;
use serde_json;
use std::fs;

use crate::scheme::Metre;

//// Structs that store the metrical data from mishra.json

#[derive(Debug, Deserialize, Clone)]
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

pub fn matches(a: &String, s: &Vec::<Metre>) -> bool {
    let m: bool = true;
    if a.len() != s.len()/4 {
        false
    } else {
        for i in 0..s.len()/4{
            //print!("{:?},{:?}", s[i].unwrap(), a.chars().nth(i).unwrap());
            //println!(" {:?}\n", s[i].unwrap() == a.chars().nth(i).unwrap() );
            if s[i].unwrap() != a.chars().nth(i).unwrap() {
                return false;
            }
        }
        return m;
    }
}

//// Function that takes the scheme as the input and returns 
/// the name of the metre

pub fn identify (s: &Vec::<Metre>) -> String {
    let result: String = String::new();
    let vrtta_kosha: vrtta_data = read_json();
    for i in 0..vrtta_kosha.metres.len(){
        let ref metre_name = vrtta_kosha.metres[i].name;
        let mut vec = Vec::new();
        match &vrtta_kosha.metres[i].pattern {
            
            StringOrList::String(a) => {
                vec.push(String::from(a));
            },
            StringOrList::List(b) => {
                vec = b.to_vec();
            },
        }

        if matches( &vec[0] , s ){
            return String::from(metre_name);
        }

    }

    result
}