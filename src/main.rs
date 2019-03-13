mod insults;

fn main() {
    let config = insults::Config::from_files();
    let insult = insults::generate_insult(config);
    println!("{}", insult);
}
