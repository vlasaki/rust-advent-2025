use std::fs;

//=== helpers ===

fn read_input(filename: &str)-> Vec<String>{
  fs::read_to_string(filename).unwrap()
    .lines()
    .map(|l| String::from(l))
    .collect()
}

fn process_input(lines: &Vec<String>)-> Vec<(i64,i64)> {
  let mut pairs = vec![];
  for line in lines {
    let (dir_str, step_str) = line.split_at(1);
    let dir = match dir_str {
      "L" => -1,
      "R" => 1,
      _ => panic!("invalid dir"),
    };
    let step = step_str.parse().unwrap();
    pairs.push((dir,step));
  }
  pairs
}

//=== dial code ===

// calculate times dial equals zero
pub fn calc_code_equals_zero(filename: &str)-> i64 {
  let lines = read_input(filename);
  let pairs = process_input(&lines);
  
  let total_dials = 100;
  let mut curr_dial = 50;
  let mut count_zero = 0;
  for (dir, step) in pairs {
    if dir < 0 {
      curr_dial -= step;
    } else {
      curr_dial += step;
    }

    curr_dial %= total_dials;

    if curr_dial == 0 {
      count_zero += 1;
    }
  }
  count_zero
}

// calculate times dial crosses zero
pub fn calc_code_crosses_zero(filename: &str)-> i64 {
  let lines = read_input(filename);
  let pairs = process_input(&lines);
  
  let total_dials = 100;
  let mut curr_dial = 50;
  let mut count_zero = 0;
  for (dir, step) in pairs {
    for _i in 0..step {
      if dir < 0 {
        curr_dial -= 1;
      } else {
        curr_dial += 1;
      }
      curr_dial %= total_dials;

      if curr_dial == 0 {
        count_zero += 1;
      }
    }
  }
  count_zero
}


//=== test dial code ===

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_calc_code_equals_zero() {
    let inp_file = "data/inp_01.csv";
    let code  = calc_code_equals_zero(&inp_file);
    //println!("equals zero code = {}", code);
    assert!(code == 1021);
  }

  #[test]
  fn test_calc_code_crosses_zero() {
    let inp_file = "data/inp_01.csv";
    let code  = calc_code_crosses_zero(&inp_file);
    //println!("crosses zero code = {}", code);
    assert!(code == 5933);
  }

}
