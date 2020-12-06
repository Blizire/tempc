/* name = tempc.rust
 * auth = trenton stiles
 * desc = convert farenheit to celsius.
 */

use std::io;

fn main() {
    println!("tempc - farenheit to celsius converter");
    println!("--------------------------------------");

    println!("input farenheit:");
    let mut f =   String::new();
    match io::stdin().read_line(&mut f){
        Ok(_bytes) => _bytes,
        Err(e) => panic!("error: {}", e),
    };

    let c: f32 = match f.trim().parse::<f32>(){
        Ok(n) => (n-32.0) * 5.0/9.0,
        Err(e) => panic!("Could not convert temp.\n{}", e),
    };

    println!("{}f is {}c", f.lines().next().unwrap(), c);
}
