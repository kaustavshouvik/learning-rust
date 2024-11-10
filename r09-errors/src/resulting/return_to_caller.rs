use std::{
    fs::File,
    io::{self, Read},
};

pub fn return_to_caller() {
    let contents = get_file_contents();
    dbg!(&contents);
}

fn get_file_contents() -> Result<String, io::Error> {
    // If file open returns an Err() variant.
    // The '?' returns the Err() variant back to the caller.

    let mut file = File::open("text.txt")?;
    let mut contents = String::new();

    // If open succeeded but reading contents returns Err(),
    // then that Err() would be returned back to the caller.
    file.read_to_string(&mut contents)?;

    // Note that the return type of function where '?' is used,
    // is important, as '?' is used to do early return.
    // Incompatible return type will result in compilation error.

    // The '?' can only be used with type which implements 'FromResidual',
    // like Option or Result.

    // For Option, if an operation results to None, it returns None
    // from the caller function else works similar to Result's Ok().
    // Also the caller function's return type must be an Option.

    // You can chain '?' calls ->
    // File::open("text.txt")?.read_to_string(&mut contents)?;

    return Ok(contents);
}
