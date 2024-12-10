use std::iter::zip;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use anyhow::{anyhow, Result};

fn get_lists(
    path: &str,
) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    let file = match File::open(path) {
        Ok(f) => f,
        Err(msg) => { return Err(anyhow!
                    ("Could not open `{path}`: {msg}!"))
                  }
    };
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let nums = line?.split("   ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        list1.push(nums[0]);
        list2.push(nums[1]);
    }
    list1.sort(); list2.sort();

    Ok((list1, list2))
}

fn get_similarity_score(
    list1: &mut Vec<i32>,
    list2: &mut Vec<i32>,
) -> Result<i32> {
    let mut scores: HashMap<i32, i32> = HashMap::new();

    list2.iter()
        .for_each(|d| { scores.entry(*d)
            .and_modify(|de| *de+=1)
            .or_insert(1); });
    
    let result = list1.iter()
        .map(|d| match scores.get_key_value(d) {
            Some((k,v)) => k*v,
            None => 0, })
        .fold(0, |res, s| res + s);
    Ok(result)
}

fn get_distance(
    list1: &mut Vec<i32>, 
    list2: &mut Vec<i32>,
) -> Result<i32> {
    let result = zip(list1, list2)
        .map(|(d1, d2)| (*d1-*d2).abs())
        .fold(0, |res, d| res + d);
    Ok(result)
}

fn main() -> Result<()> {
    let (mut list1, mut list2) = get_lists("lists.txt")?;
    let distance = get_distance(&mut list1, &mut list2)?;
    let similarity_score = get_similarity_score(&mut list1, &mut list2)?;
    println!("Distance: {distance}\nSimilarity score: {similarity_score}");

    Ok(())
}
