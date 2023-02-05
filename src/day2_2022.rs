
pub fn calculate_score1(input: Vec<String>) -> u32{
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

pub fn calculate_score2(input: Vec<String>) -> u32{
    let mut score: u32 = 0;
    for line in input {    
        let line_as_vec_chars: Vec<char> = line.chars().collect();
        let opponet_play = line_as_vec_chars[0];
        let my_play = line_as_vec_chars[2];

        match my_play {
            'X' => {
                match opponet_play {
                    'A' => score += 3,
                    'B' => score += 1,
                    'C' => score += 2,
                    _ => (),
                };
            },
            'Y' => {
                let round_score = 3;
                match opponet_play {
                    'A' => score += 1 + round_score,
                    'B' => score += 2 + round_score,
                    'C' => score += 3 + round_score,
                    _ => (),
                };
            },
            'Z' => {
                let round_score = 6;
                match opponet_play {
                    'A' => score += 2 + round_score,
                    'B' => score += 3 + round_score,
                    'C' => score += 1 + round_score,
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
    fn score_calulation1() {
        let input = vec!["A Y".to_string(),
                                      "B X".to_string(),
                                      "C Z".to_string()];
        assert_eq!(calculate_score1(input),15)
    }

    #[test]
    fn score_calulation2() {
        let input = vec!["A Y".to_string(),
                                      "B X".to_string(),
                                      "C Z".to_string()];
        assert_eq!(calculate_score2(input),12)
        
    }
}