
use std::fs::File;
use std::io::Read;
use std::error::Error;


struct Dataframe {
    data: [u8; 10],
    n: usize,
}

fn read_csv(file_path: &str) -> Result<Dataframe, Box<dyn Error>> {
    let mut f = File::open(file_path)?;
    let mut buffer = [0; 10];

    let n = f.read(&mut buffer)?;


    Ok(Dataframe{
        data: buffer,
        n: n,
    })
}


fn main() {
    println!("Hello, world!");

    let df = read_csv("./in.csv");

    match df {
        Ok(d) => println!("The data: {:?}", &d.data[..d.n]),
        Err(e) => println!("Error"),
    }

}
