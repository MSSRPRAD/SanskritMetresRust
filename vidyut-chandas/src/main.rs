//// Main Function
mod process;
mod format;
mod scheme;
mod identify;

use crate::scheme::*;
use crate::format::*;

fn main(){
    //Input Verse in slp1 encoding
    let mut verse: String = String::from("asty uttarasyAM diSI devatAtmA himAlayo nAma nagADirAjaH .
    pUrvAparO toyaniDI vigAhya sTitaH pfTivyA iva mAnadaRqaH ..");

    //Print the input verse
    println!("\nInput Verse:\n{:?}\n\n", verse);

    //Get the processed verse
    verse = process::clean(verse);

    //Display the processed verse
    println!("Processed Verse:\n{:?}\n", verse);

    //Find the scheme of the verse (Matra based functionality not implemented yet)
    let s: Vec::<Metre> = find_scheme(&verse);

    //Print the scheme of the verse
    print!("Scheme:\n");
    print_scheme(&s);

    //Print the formatted verse (Only for those whose four padas have equivalent length)
    let verse_formatted = format_verse(&verse,&s );

    
    //Print the formatted verse
    println!("\n\nOutput Verse:\n{}\n", verse_formatted);

    //Read the JSON File and print the metre (if found)
    println!("Metre Name:\n{}",identify::identify(&s));
    
}