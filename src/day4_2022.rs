pub fn count_redundant_ranges(input: Vec<String>) -> u32 {
    let mut count: u32 = 0;
    for pair_ranges in input {
        let ranges:Vec<u32> = pair_ranges.split(|char| char == '-' || char == ',')
                                         .map(|str| str.parse::<u32>().unwrap())
                                         .collect();
        count += are_ranges_redundant(ranges);
    }
   count 
}

fn are_ranges_redundant(ranges: Vec<u32>) -> u32 {
    if (ranges[0] <= ranges[2] && ranges[1] >= ranges[3]) || 
       (ranges[2] <= ranges[0] && ranges[3] >= ranges[1]) {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_redundat_ranges_test() {
        let input: Vec<String> = vec!["2-4,6-8".to_string(),
                                      "2-3,4-5".to_string(),
                                      "5-7,7-9".to_string(),
                                      "2-8,3-7".to_string(),
                                      "6-6,4-6".to_string(),
                                      "2-6,4-8".to_string()];
        assert_eq!(count_redundant_ranges(input), 2);
    }    
}