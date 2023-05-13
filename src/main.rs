use std::collections::HashMap;

use image::ImageBuffer;

lazy_static::lazy_static! {
    static ref MAPPINGS39: HashMap<char, Vec<u8>> = {
        let mut map = HashMap::<char, Vec<u8>>::new();
        for line in include_str!("../mappings/code39.txt").split('\n') {
            let mut chars = line.chars();
            let key = chars.next().unwrap();

            let mut code = Vec::<u8>::new();
            for c in chars.filter(|c| !c.is_whitespace()) {
                match c {
                    'w' => {
                        code.push(0u8);
                    },
                    'b' => {
                        code.push(1u8);
                    },
                    'W' => {
                        code.append(&mut vec![0u8, 0, 0]);
                    },
                    'B' => {
                        code.append(&mut vec![1u8, 1, 1]);
                    },
                    _ => ()
                }
            }
            code.push(0u8);

            map.insert(
                key,
                code
            );
        }
        
        map
    };
}

fn main() {
    'main_loop: loop {
        println!();
        match inquire::Text::new("Enter a string to encode (\"exit\" to exit):").prompt() {
            Ok(input) => {
                if input.len() == 0 {
                    println!("[!] Specified string is empty.");
                    continue 'main_loop;
                }

                if input == "exit" {
                    break 'main_loop;
                }

                let mut code39 = Vec::<u8>::new();
                code39.append(&mut MAPPINGS39[&'*'].clone());
                for c in input.chars() {
                    let Some(char_encoding) = MAPPINGS39.get(&c) else {
                        println!("[!] String contains illegal characters (not supported by the Code 39 [https://en.wikipedia.org/wiki/Code_39]).");
                        continue 'main_loop;
                    };
    
                    code39.append(&mut char_encoding.clone());
                }
                code39.append(&mut MAPPINGS39[&'*'].clone());
                code39.pop();
    
                let image = ImageBuffer::from_fn(
                    code39.len() as u32, 
                    50,
                    |x, _| {
                        if code39[x as usize] == 0u8 { image::Luma([255u8]) } else { image::Luma([0u8]) }
                    }
                );
                
                let file_name = format!("{}_as_barcode.png", &input);
                image.save(&file_name).unwrap();
                println!("[0] String encoded successfully and saved as \"{}\" :)", file_name);
            },
            Err(_) => (),
        };
    }
}


