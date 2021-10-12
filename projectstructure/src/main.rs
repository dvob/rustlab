mod util1 {
    pub fn run() {
        println!("util1 in src/main.rs")
    }
}

mod util2;
mod util3;

fn main() {
    util1::run();
    util2::run();
    util3::run();
    util3::sub::run();
}