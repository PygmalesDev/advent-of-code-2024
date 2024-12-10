use std::fs::File;
use std::io::prelude::*;
use anyhow::{anyhow, Result};
use regex::Regex;

fn get_text(
    path: &str,
) -> Result<String> {
    let mut text = String::new();
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(msg) => { return Err(anyhow!
                    ("Could not open `{path}`: {msg}!"))
                  }
    };
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn find_sum(
    text: &mut String
) -> Result<i32> {
    let rx = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    
    let result = rx.find_iter(text)
        .map(|exp| exp.as_str())
        .map(|exp| &exp[4..exp.len()-1])
        .map(|exp| exp.split(',')
            .map(|num| num.parse::<i32>().unwrap())
            .reduce(|a, b| a*b).unwrap())
        .sum::<i32>();
    Ok(result)
}

fn main() -> Result<()> {
    let mut text = get_text("data.txt")?;
    let result = find_sum(&mut text)?;
    println!("Programm sum: {result}");

    Ok(())
}
