use std::{ptr, io};
use std::collections::{HashSet};


fn password_checker(s: String) {
  let mut guesses = 0;
  loop {
    let mut buffer = String::new();
    if let Err(_) = io::stdin().read_line(&mut buffer) { return; }
    if buffer.len() == 0 { return; }

    // If the buffer is "Password1" then print "You guessed it!" and return,
    // otherwise print the number of guesses so far.
    println!("buffer contents {}", &buffer);
    if &buffer[0..buffer.len() -1] == &s {
      println!("You guessed it!");
      return;
    } else {
      println!("Try again! you have {} guess", guesses);
      guesses = guesses + 1
    }
  }
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
  v.iter().map(|x| {x + n}).collect()
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
  for elem in v {
    *elem = *elem + n
  }

}

fn dedup(v: &mut Vec<i32>) {
  let mut hash_set  = HashSet::new();
  let mut duplicated_indices = Vec::new();

  for index in 1..(v.len()) {
    let value = v[index];
    if hash_set.contains( &value) {
      duplicated_indices.push(index)
    } else {
      hash_set.insert(value);
      ()
    }
  }

  duplicated_indices.reverse();
  duplicated_indices.iter().for_each(|value| {println!("Duplicated indices: {}", value)} );
  duplicated_indices.iter().for_each(|index| {
    v.remove(*index);
    ()
  } )
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_password_checker() {
    password_checker(String::from("Password1"));
  }

  #[test]
  fn test_add_n() {
    assert_eq!(add_n(vec![1], 2), vec![3]);
  }

  #[test]
  fn test_add_n_inplace() {
    let mut v = vec![1];
    add_n_inplace(&mut v, 2);
    assert_eq!(v, vec![3]);
  }

  #[test]
  fn test_dedup() {
    let mut v = vec![3, 1, 0, 1, 4, 4];
    dedup(&mut v);
    assert_eq!(v, vec![3, 1, 0, 4]);
  }
}
