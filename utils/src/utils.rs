// Utility Functions file
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;    

//returns a buffered reader of a file
pub fn read_file<P>(filename: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    //open the file
    let file = File::open(filename)?;
    //if the file open was OK, return a buffered reader
    Ok(io::BufReader::new(file))
}

//reads a file and puts each line into a vec entry 
pub fn read_to_str_vec<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    //open the file
    let file = File::open(filename).expect("Bad Filename");
    //create a buffered reader to read each line
    let reader = io::BufReader::new(file);
    //get the lines, unwrap them, and collect them into a Vec<String>
    reader.lines().map(|s| s.unwrap()).collect()
}