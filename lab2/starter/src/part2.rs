use std::collections::{HashMap, HashSet};

#[derive(Clone, Eq, PartialEq)]
struct Event {
  pub name: String,
  pub month: u8,
  pub day: u8,
  pub year: u32
}

/* You need to complete two functions in this implementation
 * has_conflict() and update_event(). Note that the argument(s) and
 * return values for these two functions are missing below.
 * You can refer to tests for more information. */
impl Event {
  pub fn new(name: String, month: u8, day: u8, year: u32) -> Event {
    Event { name, month, day, year }
  }

  /* This function checks if two events are one the same date */
  pub fn has_conflict(&self, other_event: & Event) -> bool {
        self.month == other_event.month &&
        self.day == other_event.day &&
        self.year == other_event.year
    // Your code!
  }

  /* This function shifts the date of an event by one day.
   * You can assume that the date is not on the last day
   * of a month */
  pub fn update_event(&mut self) {
    // Your code!
    self.day = self.day + 1

  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Trie {
  chr: char,
  has: bool,
  children: Vec<Trie>,
}

/* ["a", "cc", "ab"] =>
   {'\0', false, [
     {'a', true, [{'b', true, []}]},
     {'c', false, [{'c', true, []}]}
   ]}
*/

impl Trie {
  pub fn new(strs: Vec<&str>) -> Trie {
    Trie::build(strs)
  }


  fn build(strs: Vec<&str>) -> Trie {
    // convert all the strs into strings
    // use a hashmap to group all the string together based on the first character
    // for each grouppin
    // Have an index pointing to where should
    // Any of the last character of any element equals to char

    let mut children = Vec::new();

    for str in strs {
      let init = (&children, str.len() );
      let result: (&Vec<Trie>, usize) = str.chars()
          .fold(init,
                |(children, remaining_chars), chr| {
        let mut new_node = match children.iter().find(|trie| {
          let trie = *trie;
          trie.chr == chr
      }) {
        Some(trie) => trie,
        None => &(Trie { chr, has: false, children: Vec::new() })
      };
      if remaining_chars == 1 {
        new_node.has = true
      };
        (&new_node.children, remaining_chars - 1)
    });
      ()
    };

    Trie {chr :'\0', has: false, children}

  }

  pub fn contains(&self, s: &str) -> bool {
    if s.len() == 0 {
      self.has
    } else {
      self.children.iter().find(|child| {
        child.chr == s.chars().next().unwrap()
      }).map(|child| {
        child.contains(&s[1..])
      }).is_some()
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_event() {
    let event1 = Event::new("Pac-12 Championship".into(), 12, 1, 2017);
    let mut event2 = Event::new("Group Project Meeting".into(), 12, 1, 2017);
    assert!(event1.has_conflict(&event2));

    event2.update_event();
    assert_eq!(event2.day, 2);
  }

  #[test]
  fn test_trie() {
//    "hello".get(10)
    let trie = Trie::new(vec!["b", "ab"]);
    assert_eq!(trie.contains("ab"), true);
    assert_eq!(trie.contains("ac"), false);
    assert_eq!(trie.contains("a"), false);
    assert_eq!(trie.contains("b"), true);
  }
}
