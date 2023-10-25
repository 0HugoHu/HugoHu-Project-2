// main.rs

use project2::run;

fn main() {
    if let Err(err) = run("mysql://root:qwerty9870@localhost:3306/mydb") {
        eprintln!("Error: {}", err);
    }
}
