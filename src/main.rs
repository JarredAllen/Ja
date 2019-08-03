#[macro_use]
extern crate clap;

fn main() {
    use clap::App;
    
    let yml = load_yaml!("arguments.yml");
    let m = App::from_yaml(yml).get_matches();
}
