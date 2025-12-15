use std::fs;
use std::collections::HashSet;

//=== helpers ===

fn process_input(filename: &str)-> Vec<(u64,u64)> {
  //parse contents in line
  let contents = fs::read_to_string(filename).unwrap();

  //parse pairs in contents
  let mut pairs = vec![];
  for pair in contents.split(',') {
    let pair = pair.trim();
    let mut parts = pair.split('-');
    let first = parts.next().unwrap().parse::<u64>().unwrap();
    let second = parts.next().unwrap().parse::<u64>().unwrap();
    pairs.push((first, second));
  }
  pairs
}

//=== invalid input sum ===

pub fn calc_invalid_input_sum_single_pattern(filename: &str)-> u64 {
  let pairs = process_input(filename);

  let mut sum_invalid = 0;
  for (first, second) in pairs {
    //first half of first number
    let first_str = first.to_string();
    let half_of_first: u64 = first_str[0..first_str.len() / 2].parse().unwrap_or(0);

    //first half of second number
    let second_str = second.to_string();
    let mut half_of_second: u64 = second_str[0..second_str.len() / 2].parse().unwrap_or(0);

    //multiply half of second by 10
    if half_of_second < half_of_first {
      half_of_second *= 10;
    }

    //iterate from half of first to half of second
    for half_id in half_of_first..=half_of_second {
      let id = format!("{}{}", half_id, half_id).parse::<u64>().unwrap_or(0);
      if id >= first && id <= second {
        sum_invalid += id;
      }
    }
  }
  sum_invalid
}


pub fn calc_invalid_input_sum_multi_pattern(filename: &str)-> u64 {
  let pairs = process_input(filename);

  let mut sum_invalid = 0;
  for (first,second) in pairs {
    let mut invalid_ids = HashSet::<u64>::new();
    //first half of first number
    let first_str = first.to_string();
    let half_of_first: u64 = if first_str.len() > 1 {
      first_str[0..first_str.len() / 2].parse().unwrap_or(0)
    } else {
      first
    };

    //first half of second number
    let second_str = second.to_string();
    let mut half_of_second: u64 = if second_str.len() > 1 {
      second_str[0..second_str.len() / 2].parse().unwrap_or(0)
    } else {
      second
    };

    //multiply half of second by 10
    if half_of_second < half_of_first {
      half_of_second *= 10;
    }

    //iterate from half of first to half of second
    for half_id in half_of_first..=half_of_second {
      let half_str = half_id.to_string();
      //test shorter seqs from this half
      for len in (1..=half_str.len()).rev() {
        let seq = &half_str[0..len];
        let seq_num = seq.parse::<u64>().unwrap_or(0);

        //convert to number
        let digits = seq_num.ilog10() + 1;
        let multiplier = 10u64.pow(digits);
        let mut id: u64 = seq_num * multiplier + seq_num;
        while id <= second {
          if id >= first && id <= second {
            invalid_ids.insert(id);
          }

          id = id * multiplier + seq_num;
        }
      }
    }

    sum_invalid += invalid_ids.iter().sum::<u64>();
  }


  sum_invalid
}



//=== test ===

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_invalid_input_sum_single_pattern() {
    let input_file = "data/inp_02.csv";
    let sum = calc_invalid_input_sum_single_pattern(input_file);
    //println!("invalid input sum = {}", sum);
    assert!(sum == 26255179562); 
  }

  #[test]
  fn test_invalid_input_sum_multi_pattern() {
    let input_file = "data/inp_02.csv";
    let sum = calc_invalid_input_sum_multi_pattern(input_file);
    //println!("invalid input sum = {}", sum);
    assert!(sum == 31680313976); 
  }

}

