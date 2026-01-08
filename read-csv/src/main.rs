use std::error::Error;
use csv::{ReaderBuilder, StringRecord};

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{
    let reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(path);

    for result in reader?.records(){
        let record: StringRecord = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main () {
   match read_from_file("./dummy.csv"){
        // eprintln used for printing errors and progress messages
        Ok(_) => println!("File conpressed successfully"),
        Err(e) => eprintln!("Error {:}", e)
    }
}


