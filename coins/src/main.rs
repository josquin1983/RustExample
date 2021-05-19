extern crate libcoin;

extern crate data_fromfilelib;

use libcoin::coin::{is_bad_coin, Coin}; 

use data_fromfilelib::fromfile;

use std::env;

     fn main() {
    

    let mut coins = Vec::with_capacity(20);

    let args: Vec<String> = env::args().collect();

     match args.len() {

     1 => panic!("There are not enough arguments. Enter the filename that contains the data!"),

     2 => {}, 

     _ => panic!("Too much arguments. Enter the filename only that contains the data!"),

     }


    let file_name = &args[1];

    let vc_data: Vec<f32> = match fromfile::read_from_file(file_name) {
        Ok(data) => data,
        Err(e) => panic!("The program hasn't read the data from this file correctly. The error {:?} occurs.", e),
    };

    for item in vc_data.into_iter() {

        println!("{:?}", item);
        coins.push(Coin::new(item));
    }


    if is_bad_coin(&coins) {
        println!("Данные в файле говорят о наличии нестандартной монеты");
    } else {
        println!("Нестандарные экземпляры отсутствуют");
    }
}
