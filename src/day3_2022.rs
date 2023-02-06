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
    use super::{sum_common_item_priority, get_priority};


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
        assert_eq!(sum_common_item_priority(input),16+38);
    }
}

