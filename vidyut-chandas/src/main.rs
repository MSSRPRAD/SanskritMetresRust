//// Main Function
mod format;
mod identify;
mod process;
mod scheme;

use crate::format::*;
use crate::scheme::*;

fn main() {
    //Input Verse in slp1 encoding
    let mut verse: String = String::from(
        "vAgarTAv iv saMpfktO vAgarTapratipattaye .
jagataH tarO vande pArvatIparmeSvarO ..",
    );

    //Print the input verse
    println!("\nInput Verse:\n{:?}\n\n", verse);

    //Get the processed verse
    verse = process::clean(verse);

    //Display the processed verse
    println!("Processed Verse:\n{:?}\n", verse);

    //Find the scheme of the verse (Matra based functionality not implemented yet)
    let s: Vec<Metre> = find_scheme(&verse);

    //Print the scheme of the verse
    print!("Scheme:\n");
    print_scheme(&s);

    //Print the formatted verse (Only for those whose four padas have equivalent length)
    let verse_formatted = format_verse(&verse, &s);

    //Print the formatted verse
    println!("\n\nOutput Verse:\n{}\n", verse_formatted);

    //Read the JSON File and print the metre (if found)
    println!("Metre Name:\n{}", identify::identify(&s));

    println!("\n\n");
}
