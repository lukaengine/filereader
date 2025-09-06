use std::fs::File;
use std::io::{self, Write, Read};
use std::fs;


fn main() {
    
    loop {
        println!("Please enter an action.");
        println!("1. Create a new file");
        println!("2. Read a file");
        println!("3. Write to a file");
        println!("4. Delete a file");
        println!("5. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let input = input.trim();

        match input {
            "1." => createfile(),
            "2." => readfile(),
            "3." => writefile(),
            _ => println!("Okay!")

        }


    }
}

fn createfile() {
    println!("Please input the file name");
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name);
    let file_name = file_name.trim();

    let mut file = File::create(file_name).unwrap();

    println!("Would you like to add any text to the file? (yes/no) ");

    let mut opinion = String::new();

    io::stdin().read_line(&mut opinion);
    let opinion = opinion.trim();

    if opinion == "yes" {
        println!("enter file contents");
        let mut file_contents = String::new();
        io::stdin().read_line(&mut file_contents);
        let file_contents = file_contents.trim().as_bytes();

        file.write(file_contents).expect("Failed to write file contents!");
       
        
    }



}


fn readfile() {
    let mut variable = String::new();
    println!("please enter the file you wish to read.");
    io::stdin().read_line(&mut variable);

    let variable = variable.trim();

    let contents = fs::read_to_string(variable).unwrap();

    

    println!("{}", contents);




}

fn writefile() {
    println!("please select which fil =e you wish to write to");
    let mut text = String::new();
    io::stdin().read_line(&mut text);

    let text = text.trim();

    println!("now write the u wish in there");
    let mut the_write = String::new();
    io::stdin().read_line(&mut the_write);

    let the_write = the_write.trim();

    let mut file = File::create(text).unwrap();
    file.write_all(the_write.as_bytes()).unwrap();

    


}