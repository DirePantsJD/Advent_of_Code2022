
pub fn calculate_score(input: Vec<String>) -> u32{
    let mut score: u32 = 0;
    for line in input {    
        let line_as_vec_chars: Vec<char> = line.chars().collect();
        let opponet_play = line_as_vec_chars[0];
        let my_play = line_as_vec_chars[2];

        match my_play {
            'X' => {
                let shape_score = 1;
                match opponet_play {
                    'A' => score += 3 + shape_score,
                    'B' => score += shape_score,
                    'C' => score += 6 + shape_score,
                    _ => (),
                };
            },
            'Y' => {
                let shape_score = 2;
                match opponet_play {
                    'A' => score += 6 + shape_score,
                    'B' => score += 3 + shape_score,
                    'C' => score += shape_score,
                    _ => (),
                };
            },
            'Z' => {
                let shape_score = 3;
                match opponet_play {
                    'A' => score += shape_score,
                    'B' => score += 6 + shape_score,
                    'C' => score += 3 + shape_score,
                    _ => (),
                };
            }
            _ => (),
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_calulation() {
        let input = vec!["A Y".to_string(),
                                      "B X".to_string(),
                                      "C Z".to_string()];
        assert_eq!(calculate_score(input),15)
    }
}