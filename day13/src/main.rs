use std::{cmp::Ordering, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Result of part 1: {}", part1(&input));
    println!("Result of part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    index_sum_of_rightly_ordered_pairs_from_str(input)
}

fn part2(input: &str) -> usize {
    let product = get_product_of_indices_for_dividers(input, "[[2]]", "[[6]]");

    product
}

#[derive(Debug, Clone)]
enum Item {
    Number(i32),
    List(Vec<Item>),
}

impl Item {
    fn is_right_order(&self, item: &Item) -> (bool, Vec<String>) {
        #[cfg(test)]
        println!("Comparing {:?} and {:?}", self, item);

        match (self, item) {
            (Item::Number(left), Item::Number(right)) => {
                if left <= right {
                    (true, vec![])
                } else {
                    (false, vec![format!("{} > {}", left, right)])
                }
            }
            (Item::Number(left), Item::List(_)) => ln(*left).is_right_order(item),
            (Item::List(_), Item::Number(right)) => self.is_right_order(&ln(*right)),
            (Item::List(left), Item::List(right)) => {
                if left.is_empty() {
                    #[cfg(test)]
                    println!("left is empty");
                    return (true, vec![]);
                }

                if right.is_empty() {
                    #[cfg(test)]
                    println!("right is empty");
                    return (false, vec![]);
                }

                let left_front = &left[0];
                let left_rest = l(left[1..].to_vec());
                let right_front = &right[0];
                let right_rest = l(right[1..].to_vec());

                let left_is_right_order = left_front.is_right_order(right_front).0;
                let right_is_right_order = right_front.is_right_order(left_front).0;

                match (left_is_right_order, right_is_right_order) {
                    (true, true) => { // both are equal, continue
                    }
                    (true, false) => {
                        // left is smaller than right, return true
                        return (true, vec![]);
                    }
                    (false, true) => {
                        // left is larger than right, return false
                        return (false, vec![]);
                    }
                    (false, false) => {
                        panic!("Logical error: both list fronts are not in order in any way");
                    }
                }

                // convert left_rest to Vec<Item>
                left_rest.is_right_order(&right_rest)
            }
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.is_right_order(other).0 && other.is_right_order(self).0
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left_is_right = self.is_right_order(other).0;
        let right_is_right = other.is_right_order(self).0;

        match (left_is_right, right_is_right) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            _ => None,
        }
    }
}

impl Eq for Item {}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn index_sum_of_rightly_ordered_pairs_from_str(input: &str) -> usize {
    let pairs = split_to_vec_of_pairs(input);

    let rightly_ordered = get_rightly_ordered_pairs(&pairs);

    let mut sum = 0;

    for (i, is_rightly_ordered) in rightly_ordered.iter().enumerate() {
        if *is_rightly_ordered {
            sum += i + 1;
        }
    }

    sum
}

fn get_rightly_ordered_pairs(pairs: &[(Item, Item)]) -> Vec<bool> {
    pairs.iter().map(|(a, b)| a.is_right_order(b).0).collect()
}

fn split_to_vec_of_pairs(input: &str) -> Vec<(Item, Item)> {
    let mut pairs: Vec<(Item, Item)> = Vec::new();

    for line in input.split("\n\n") {
        let mut lines = line.split('\n');
        let a = Item::from_str(lines.next().unwrap());
        let b = Item::from_str(lines.next().unwrap());
        pairs.push((a, b));
    }

    pairs
}

#[cfg(test)]
fn n(number: i32) -> Item {
    Item::Number(number)
}

fn l(list: Vec<Item>) -> Item {
    Item::List(list)
}

fn ln(number: i32) -> Item {
    Item::List(vec![Item::Number(number)])
}

enum Token {
    OpenBracket,
    CloseBracket,
    Number(i32),
}

impl Item {
    #[allow(dead_code)]
    fn from_str(input: &str) -> Item {
        let mut tokens: Vec<Token> = Vec::new();

        let mut continued_token = String::new();

        for char in input.trim().chars() {
            match char {
                '[' => {
                    if !continued_token.is_empty() {
                        tokens.push(Token::Number(continued_token.parse().unwrap()));
                        continued_token = String::new();
                    }
                    tokens.push(Token::OpenBracket);
                }
                ']' => {
                    if !continued_token.is_empty() {
                        tokens.push(Token::Number(continued_token.parse().unwrap()));
                        continued_token = String::new();
                    }
                    tokens.push(Token::CloseBracket);
                }
                ',' => {
                    if !continued_token.is_empty() {
                        tokens.push(Token::Number(continued_token.parse().unwrap()));
                        continued_token = String::new();
                    }
                }
                '0'..='9' => continued_token.push(char),
                _ => panic!("Invalid character: {}", char),
            }
        }

        if !continued_token.is_empty() {
            tokens.push(Token::Number(continued_token.parse().unwrap()));
        }

        let opening_bracket_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::OpenBracket))
            .count();
        let closing_bracket_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::CloseBracket))
            .count();

        assert_eq!(opening_bracket_count, closing_bracket_count);

        assert!(!tokens.is_empty());

        Item::from_tokens(tokens)
    }

    fn from_tokens(tokens: Vec<Token>) -> Item {
        let mut item_stack: Vec<Item> = Vec::new();

        if tokens.len() == 1 {
            match tokens[0] {
                Token::Number(number) => return Item::Number(number),
                _ => panic!("Invalid solitary token, which is not a number"),
            }
        }

        for token in tokens {
            match token {
                Token::OpenBracket => {
                    item_stack.push(Item::List(Vec::new()));
                }
                Token::Number(number) => match item_stack.last_mut() {
                    Some(Item::List(list)) => {
                        list.push(Item::Number(number));
                    }
                    _ => panic!("Unexpected number token, which is not in a list"),
                },
                Token::CloseBracket => {
                    // push the current top item onto the one before it
                    let item = item_stack.pop().unwrap();
                    match item_stack.last_mut() {
                        Some(Item::List(list)) => {
                            list.push(item);
                        }
                        None => return item,
                        _ => panic!("Unexpected closing bracket, which is not in a list"),
                    }
                }
            }
        }

        panic!("Unexpected end of tokens without final closing bracket, maybe");
    }
}

fn get_vec_of_items_from_str(input: &str) -> Vec<Item> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Item::from_str)
        .collect()
}
fn get_product_of_indices_for_dividers(
    input: &str,
    divider_packet_1: &str,
    divider_packet_2: &str,
) -> usize {
    let mut items = get_vec_of_items_from_str(input);

    let divider_item_1 = Item::from_str(divider_packet_1);
    let divider_item_2 = Item::from_str(divider_packet_2);

    items.push(divider_item_1.clone());
    items.push(divider_item_2.clone());

    // sort items
    items.sort();

    // get position of divider_item_1 and divider_item_2 in sorted items
    let divider_item_1_index = items
        .iter()
        .position(|item| item == &divider_item_1)
        .unwrap()
        + 1;
    let divider_item_2_index = items
        .iter()
        .position(|item| item == &divider_item_2)
        .unwrap()
        + 1;

    let product = divider_item_1_index * divider_item_2_index;

    product
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_part1() {
        assert_eq!(n(1), n(1));

        assert_eq!(n(1), ln(1));

        assert_eq!(n(1) <= n(1), true);
        assert_eq!(n(1) <= n(2), true);
        assert_eq!(n(2) <= n(1), false);

        assert_eq!(l(vec![]) <= l(vec![]), true);

        assert_eq!(l(vec![]) <= l(vec![n(1)]), true);

        assert_eq!(l(vec![n(1)]) <= l(vec![]), false);

        assert_eq!(Item::from_str("1"), n(1));
        assert_eq!(Item::from_str("2"), n(2));
        assert_eq!(Item::from_str("[]"), l(vec![]));
        assert_eq!(Item::from_str("[1]"), ln(1));
        assert_eq!(Item::from_str("[1]"), ln(1));

        assert!(Item::from_str("1") == Item::from_str("1"));
        assert!(Item::from_str("1") <= Item::from_str("1"));

        assert!(Item::from_str("[1,1,3,1,1]") <= Item::from_str("[1,1,5,1,1]"));

        assert!(Item::from_str("[[1],[2,3,4]]") <= Item::from_str("[[1],4]"));

        assert_eq!(Item::from_str("9"), Item::Number(9));
        assert_eq!(Item::from_str("[8]"), Item::List(vec![Item::Number(8)]));

        assert_eq!(ln(9).is_right_order(&ln(8)).0, false);
        assert!(ln(9) > ln(8));

        assert!(Item::from_str("9") > Item::from_str("[8]"));

        assert!(Item::from_str("[9]") > Item::from_str("[[8,7,6]]"));

        assert!(Item::from_str("[7,7,7,7]") > Item::from_str("[7,7,7]"));

        let pairs = split_to_vec_of_pairs(INPUT);

        let rightly_ordered = get_rightly_ordered_pairs(&pairs);

        assert_eq!(
            rightly_ordered,
            vec![true, true, false, true, false, true, false, false]
        );

        assert_eq!(index_sum_of_rightly_ordered_pairs_from_str(INPUT), 13);
    }

    #[test]
    fn test_part2() {
        let product =
            get_product_of_indices_for_dividers(INPUT, DIVIDER_PACKET_1, DIVIDER_PACKET_2);

        assert_eq!(product, 140);
    }

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    const _REFERENCE_OUTPUT: &str = "== Pair 1 ==
- Compare [1,1,3,1,1] vs [1,1,5,1,1]
  - Compare 1 vs 1
  - Compare 1 vs 1
  - Compare 3 vs 5
    - Left side is smaller, so inputs are in the right order

== Pair 2 ==
- Compare [[1],[2,3,4]] vs [[1],4]
  - Compare [1] vs [1]
    - Compare 1 vs 1
  - Compare [2,3,4] vs 4
    - Mixed types; convert right to [4] and retry comparison
    - Compare [2,3,4] vs [4]
      - Compare 2 vs 4
        - Left side is smaller, so inputs are in the right order

== Pair 3 ==
- Compare [9] vs [[8,7,6]]
  - Compare 9 vs [8,7,6]
    - Mixed types; convert left to [9] and retry comparison
    - Compare [9] vs [8,7,6]
      - Compare 9 vs 8
        - Right side is smaller, so inputs are not in the right order

== Pair 4 ==
- Compare [[4,4],4,4] vs [[4,4],4,4,4]
  - Compare [4,4] vs [4,4]
    - Compare 4 vs 4
    - Compare 4 vs 4
  - Compare 4 vs 4
  - Compare 4 vs 4
  - Left side ran out of items, so inputs are in the right order

== Pair 5 ==
- Compare [7,7,7,7] vs [7,7,7]
  - Compare 7 vs 7
  - Compare 7 vs 7
  - Compare 7 vs 7
  - Right side ran out of items, so inputs are not in the right order

== Pair 6 ==
- Compare [] vs [3]
  - Left side ran out of items, so inputs are in the right order

== Pair 7 ==
- Compare [[[]]] vs [[]]
  - Compare [[]] vs []
    - Right side ran out of items, so inputs are not in the right order

== Pair 8 ==
- Compare [1,[2,[3,[4,[5,6,7]]]],8,9] vs [1,[2,[3,[4,[5,6,0]]]],8,9]
  - Compare 1 vs 1
  - Compare [2,[3,[4,[5,6,7]]]] vs [2,[3,[4,[5,6,0]]]]
    - Compare 2 vs 2
    - Compare [3,[4,[5,6,7]]] vs [3,[4,[5,6,0]]]
      - Compare 3 vs 3
      - Compare [4,[5,6,7]] vs [4,[5,6,0]]
        - Compare 4 vs 4
        - Compare [5,6,7] vs [5,6,0]
          - Compare 5 vs 5
          - Compare 6 vs 6
          - Compare 7 vs 0
            - Right side is smaller, so inputs are not in the right order";

    const DIVIDER_PACKET_1: &str = "[[2]]";
    const DIVIDER_PACKET_2: &str = "[[6]]";
}
