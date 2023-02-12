pub fn num_tail_positions(input: Vec<String>) -> u32 {
    let mut count = 1;
    let mut passed: Vec<(i16,i16)> = vec![(0,0)];
    let mut head_pos: (i16,i16) = (0,0);
    let mut prev_h_pos: (i16,i16);
    let mut tail_pos: (i16,i16) = (0,0);

    for command in input {
        let mut iter = command.split(" ");
        let direction = iter.next().unwrap();
        let quant: i16 = iter.next().unwrap().parse().unwrap();
        match direction {
            "R" => {
                for _ in 0..quant {
                    prev_h_pos = head_pos;
                    head_pos.0 += 1;
                    if adjust_tail(&mut tail_pos, &head_pos, &prev_h_pos) {
                        if !passed.contains(&tail_pos) {
                            passed.push(tail_pos);
                            count+=1
                        }
                    }
                }
            },
            "L" => {
                for _ in 0..quant {
                    prev_h_pos = head_pos;
                    head_pos.0 -= 1;
                    if adjust_tail(&mut tail_pos, &head_pos, &prev_h_pos) {
                        if !passed.contains(&tail_pos) {
                            passed.push(tail_pos);
                            count+=1
                        }
                    }
                }
            },
            "U" => {
                for _ in 0..quant {
                    prev_h_pos = head_pos;
                    head_pos.1 += 1;
                    if adjust_tail(&mut tail_pos, &head_pos, &prev_h_pos) {
                        if !passed.contains(&tail_pos) {
                            passed.push(tail_pos);
                            count+=1
                        }
                    }
                }
            },
            "D" => {
                for _ in 0..quant {
                    prev_h_pos = head_pos;
                    head_pos.1 -= 1;
                    if adjust_tail(&mut tail_pos, &head_pos, &prev_h_pos) {
                        if !passed.contains(&tail_pos) {
                            passed.push(tail_pos);
                            count+=1
                        }
                    }
                }
            },
            _ => ()
        }
    }

    count
}

fn adjust_tail(tail_pos: &mut (i16,i16), head_pos: &(i16,i16), prev_h_pos: &(i16,i16)) -> bool {
    let diff = ((head_pos.0 - tail_pos.0).abs(), (head_pos.1 -tail_pos.1).abs()); 
    if diff.0 > 1 || diff.1 > 1 {
       *tail_pos = *prev_h_pos;
       return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn adjust_tail_eq() {
        let mut tail_pos: (i16,i16) = (3,3);
        adjust_tail(&mut tail_pos, &(4,4), &(3,4));
        assert_eq!(tail_pos, (3,3));
    }
    #[test] 
    fn adjust_tail_change() {
        let mut tail_pos: (i16,i16) = (3,3);
        adjust_tail(&mut tail_pos, &(3,5), &(3,4));
        assert_eq!(tail_pos, (3,4));
    }

    #[test]
    fn num_tail_pos_test() {
        let input = vec![
            "R 4".to_string(),
            "U 4".to_string(),
            "L 3".to_string(),
            "D 1".to_string(),
            "R 4".to_string(),
            "D 1".to_string(),
            "L 5".to_string(),
            "R 2".to_string()
        ];
        assert_eq!(num_tail_positions(input),13);
    }
}