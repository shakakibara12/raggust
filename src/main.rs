mod chunk;
mod parse;

fn main() {
    let the_silent_patient = parse::Novel::open("corpus/The_Silent_Patient.epub");

    dbg!(the_silent_patient);
}
