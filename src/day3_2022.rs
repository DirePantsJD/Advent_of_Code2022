//day1 fn
pub fn sum_common_item_priority(input: Vec<String>) -> u32 {
    let mut total = 0;
    for rucksack in input {
        let mut first_half_og = rucksack.clone();
        let second_half: Vec<char> = first_half_og.split_off(first_half_og.len() / 2).chars().collect();
        let first_half: Vec<char> = first_half_og.chars().collect();

        for char in first_half {
            let vec = second_half.iter()
                                             .filter(|&&sh_char| sh_char == char)
                                             .collect::<Vec<&char>>(); 

            if !vec.is_empty(){
                total += get_priority(*(vec[0]));
                break;
            }
        }
    }
    total
}

//day2 fn
pub fn sum_badge_priority(input: Vec<String>) -> u32 {
    let mut total: u32 = 0;
    let mut count: u8 = 0;
    let mut group = Vec::new();

    for rucksack in input {
        if count < 3 {
            group.push(rucksack);    
            count += 1;
        } else {
            total += get_badge_priority(group.clone());
            group.clear();
            group.push(rucksack);
            count = 1;
        }
    }
    total += get_badge_priority(group.clone());

    total
}

// aux fns
fn get_badge_priority(input: Vec<String>) -> u32 {
    let mut badge_priority: u32 = 0;
    for item1 in input[0].chars(){
        for item2 in input[1].chars(){
            for item3 in input[2].chars(){
                if item1 == item2 && item2 == item3{
                    badge_priority = get_priority(item1);
                }
            }
        }
    } 
    badge_priority
}

fn get_priority(ch: char) -> u32 {
    if ch.is_lowercase() {
        get_priority_lc(ch)
    } else{
        get_priority_lc(ch.to_lowercase().next().unwrap()) + 26
    }

}

fn get_priority_lc(ch: char) -> u32 {
    match ch{
        'a' => 1,'b' => 2,'c' => 3,'d' => 4,'e' => 5,
        'f' => 6,'g' => 7,'h' => 8,'i' => 9,'j' =>10,
        'k' =>11,'l' =>12,'m' =>13,'n' =>14,'o' =>15,
        'p' =>16,'q' =>17,'r' =>18,'s' =>19,'t' =>20,
        'u' =>21,'v' =>22,'w' =>23,'x' =>24,'y' =>25,'z' =>26,
         _  => 0,
    }
}



#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn split_off_test() {
        let mut og_string = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
        let second_half = og_string.split_off(og_string.len()/2);
        assert_eq!(og_string.len(),second_half.len())
    }

    #[test]
    fn get_priority_lowercase_test() {
        assert_eq!(get_priority('p'),16);
    }

    #[test]
    fn get_priority_uppercase_test() {
        assert_eq!(get_priority('L'),38);
    }

    #[test]
    fn sum_common_items_prio_test() {
        let input = vec!["vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                                      "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()];
        assert_eq!(sum_common_item_priority(input),16 + 38);
    }

    #[test]
    fn get_badge_prio_test() {
        let input = vec!["vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                                      "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                                      "PmmdzqPrVvPwwTWBwg".to_string()];
        assert_eq!(get_badge_priority(input),18);
    }

    #[test]
    fn get_badge_prio_sum_test() {
        let input = vec!["vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                                      "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                                      "PmmdzqPrVvPwwTWBwg".to_string(),
                                      "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
                                      "ttgJtRGJQctTZtZT".to_string(),
                                      "CrZsJsPPZsGzwwsLwLmpwMDw".to_string()];
        assert_eq!(sum_badge_priority(input),18 + 52);
    }

}

