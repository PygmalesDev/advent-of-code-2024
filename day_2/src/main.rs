use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use anyhow::{anyhow, Result};

fn get_reports(
    path: &str,
) -> Result<Vec<Vec<i32>>> {
    let mut reports: Vec<Vec<i32>> = vec![];

    let file = match File::open(path) {
        Ok(f) => f,
        Err(msg) => { return Err(anyhow!
                    ("Could not open `{path}`: {msg}!"))
                  }
    };
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        reports.push(line?.split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>());
    }

    Ok(reports)
}

fn count_safe_reports(
    reports: &mut Vec<Vec<i32>>,
) -> Result<i32> {
    let mut safe_reports = 0;

    for report in reports {
        let mut is_damped = false;
        let mut is_safe = true;
        let mut direction = (report[0]-report[1]).signum();
        let mut difference;
        
        for l in 0..report.len()-1 {
            difference = report[l] - report[l+1];
            
            if (difference.abs() > 3) || (direction == 0) || 
               (difference.signum() != direction) {
                if is_damped {
                    is_safe = false; 
                    break;
                } else {
                    is_damped = true;
    
                    difference = if (difference <= 3) && (difference > 0) { difference.signum()*(-1) } 
                               else if l != report.len()-2 { report[l] - report[l+2] } 
                               else { break; };
                    
                    direction = difference.signum();
                }
            }
        }
        if is_safe { safe_reports += 1; }
    }

    Ok(safe_reports)
}

fn main() -> Result<()> {
    let mut reports = get_reports("reports.txt")?;
    let safe_reports = count_safe_reports(&mut reports)?;
    println!("Safe reports (Problem Dampener active): {}/{}", safe_reports, reports.len());

    Ok(())
}
