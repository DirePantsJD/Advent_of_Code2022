use std::collections::VecDeque;

pub fn start_of_packet_offset(input: Vec<String>) -> u16 {
    let mut offset: u16 = 0;
    let mut four_chars: VecDeque<char> = VecDeque::new();
    for char in input[0].chars() {
        if four_chars.len() == 4 {
            four_chars.pop_front();
        }
        four_chars.push_back(char);
        offset+=1;
        if check_four_chars(four_chars.clone()) {
            break;
        }
    }
    offset
}

fn check_four_chars(four_chars: VecDeque<char>) -> bool {
    let lenght = four_chars.len();
    if lenght != 4 { 
        return false;
    }
    for idx1 in 0..lenght {
        if idx1 == lenght - 1 {
            return true;
        }
        for idx2 in (idx1+1)..lenght{
            if four_chars[idx1] == four_chars[idx2] {
                return false;
            }
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_four_chars_test() {
        let dequeue = VecDeque::from(vec!['a','b','c','d']);
        assert_eq!(check_four_chars(dequeue),true);
    }

    #[test]
    fn check_four_chars_small_deq() {
        let dequeue = VecDeque::from(vec!['a','b','c']);
        assert_eq!(check_four_chars(dequeue),false);
    }

    #[test]
    fn check_four_chars_empty() {
        let dequeue: VecDeque<char> = VecDeque::new();
        assert_eq!(check_four_chars(dequeue),false);
    }

    #[test]
    fn check_five_chars() {
        let dequeue = VecDeque::from(vec!['a','b','c','d','e']);
        assert_eq!(check_four_chars(dequeue),false);
    }
}