use std::io;
fn main() {
    println!("FarenCel by Lenesis\n");
    println!("Select 1 for F2C, and 2 for C2F");
    let mut mode = String::new();
    io::stdin()
        .read_line(&mut mode)
        .expect("What the fuck?!");
    let mode: u32 = match mode.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Wrong selection! choosing default F2C");
            1
        },
    };

    if mode == 1{
        let number = get_input();
        println!("{} Farenheit is {} Celesius.", number, f2c(number));
    } else if mode == 2 {
        let number = get_input();
        println!("\n{} Celesius is {} Farenheit.", number, c2f(number));
    } else {
        println!("Wrong selection! choosing default F2C");
        let number = get_input();
        println!("{} Farenheit is {} Celesius.", number, f2c(number));
    }
}
fn get_input() -> f64 {
    loop {
        println!("\nEnter the temperature:");
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("What the fuck?!");
        let inp: f64 = match inp.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Wrong number! try again.");
                continue;
            }   
        };
        break inp;
    }
}

fn f2c(frn: f64) -> f64 {
    (frn - 32.0) / 1.8
}

fn c2f(cel: f64) -> f64{
    cel * 1.8 + 32.0
}
