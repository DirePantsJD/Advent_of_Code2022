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
                    if adjust_node(&mut tail_pos, &head_pos, &prev_h_pos) {
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
                    if adjust_node(&mut tail_pos, &head_pos, &prev_h_pos) {
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
                    if adjust_node(&mut tail_pos, &head_pos, &prev_h_pos) {
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
                    if adjust_node(&mut tail_pos, &head_pos, &prev_h_pos) {
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

fn adjust_node(node_pos: &mut (i16,i16), prev_node_pos: &(i16,i16), prev_node_prev_pos: &(i16,i16)) -> bool {
    let diff = ((prev_node_pos.0 - node_pos.0), (prev_node_pos.1 - node_pos.1)); 
    let prev_node_coord_diff = ((prev_node_pos.0 - prev_node_prev_pos.0), (prev_node_pos.1 - prev_node_prev_pos.1)); 
    // dbg!(diff);
    // dbg!(prev_node_coord_diff);
    if prev_node_coord_diff.0.abs() == 1 && prev_node_coord_diff.1.abs() == 1 {
        if diff.0.abs() == 2 {
            if diff.1.abs() == 1 {
                *node_pos = (prev_node_prev_pos.0, prev_node_prev_pos.1 + diff.1); 
                return true;
            }
        } else if diff.1.abs() == 2 {
            if diff.0.abs() == 1 {
                *node_pos = (prev_node_prev_pos.0 + diff.0, prev_node_prev_pos.1); 
                return true;
            }
        }
    }
    if diff.0.abs() > 1 || diff.1.abs() > 1 {
        *node_pos = *prev_node_prev_pos;
        return true;
     }
    false
}

pub fn num_tail_positions2(input: Vec<String>) -> u32 {
    let mut count = 1;
    let mut passed: Vec<(i16,i16)> = vec![(0,0)];
    let mut positions: Vec<(i16,i16)> = vec![(0,0),(0,0),(0,0),(0,0),(0,0),
                                             (0,0),(0,0),(0,0),(0,0),(0,0)
                                            ];
    let mut prev_positions: Vec<(i16,i16)> = vec![(0,0),(0,0),(0,0),(0,0),(0,0),
                                                  (0,0),(0,0),(0,0),(0,0),(0,0)
                                                 ];

    for command in input {
        let mut iter = command.split(" ");
        let direction = iter.next().unwrap();
        let quant: i16 = iter.next().unwrap().parse().unwrap();
        match direction {
            "R" => {
                for _ in 0..quant {
                    prev_positions[0] = positions[0];
                    positions[0].0 += 1;
                    for i in 1..=9 {
                        prev_positions[i] = positions[i];
                        let prev = prev_positions[i-1];
                        let prev_node = positions[i-1];
                        let curr = &mut positions[i];
                        if adjust_node(curr, &prev_node, &prev) {
                            if !passed.contains(&positions[i]) && i==9 {
                                passed.push(positions[i]);
                                count+=1
                            }
                        }
                    }
                }
                println!("{:?}", positions.clone());
            },
            "L" => {
                for _ in 0..quant {
                    prev_positions[0] = positions[0];
                    positions[0].0 -= 1;
                    for i in 1..=9 {
                        prev_positions[i] = positions[i];
                        let prev = prev_positions[i-1];
                        let prev_node = positions[i-1];
                        let curr = &mut positions[i];
                        if adjust_node(curr, &prev_node, &prev) {
                            if !passed.contains(&positions[i]) && i==9 {
                                passed.push(positions[i]);
                                count+=1
                            }
                        }
                    }
                }
                println!("{:?}", positions.clone());
            },
            "U" => {
                for _ in 0..quant {
                    prev_positions[0] = positions[0];
                    positions[0].1 += 1;
                    for i in 1..=9 {
                        prev_positions[i] = positions[i];
                        let prev = prev_positions[i-1];
                        let prev_node = positions[i-1];
                        let curr = &mut positions[i];
                        if adjust_node(curr, &prev_node, &prev) {
                            if !passed.contains(&positions[i]) && i==9 {
                                passed.push(positions[i]);
                                count+=1
                            }
                        }
                    }
                }
                println!("{:?}", positions.clone());
            },
            "D" => {
                for _ in 0..quant {
                    prev_positions[0] = positions[0];
                    positions[0].1 -= 1;
                    for i in 1..=9 {
                        prev_positions[i] = positions[i];
                        let prev = prev_positions[i-1];
                        let prev_node = positions[i-1];
                        let curr = &mut positions[i];
                        if adjust_node(curr, &prev_node, &prev) {
                            if !passed.contains(&positions[i]) && i==9 {
                                passed.push(positions[i]);
                                count+=1
                            }
                        }
                    }
                }
                println!("{:?}", positions.clone());
            },
            _ => ()
        }
    }
    //dbg!(passed);
    count
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn adjust_node_eq() {
        let mut node_pos: (i16,i16) = (3,3);
        adjust_node(&mut node_pos, &(3,4), &(4,3));
        assert_eq!(node_pos, (3,3));
    }
    #[test] 
    fn adjust_node_change_horizontal() {
        let mut node_pos: (i16,i16) = (3,3);
        adjust_node(&mut node_pos, &(3,5), &(3,4));
        assert_eq!(node_pos, (3,4));
    }

    #[test]
    fn adjust_node_change_true_diagonal() {
        let mut node_pos: (i16,i16) = (3,3);
        adjust_node(&mut node_pos, &(5,5), &(4,4));
        assert_eq!(node_pos, (4,4));
    }

    #[test]
    fn adjust_node_change_skewed_diagonal1() {
        let mut node_pos: (i16,i16) = (3,3);
        adjust_node(&mut node_pos, &(2,5), &(3,4));
        assert_eq!(node_pos, (2,4));
    }

    #[test]
    fn adjust_node_change_skewed_diagonal2() {
        let mut node_pos: (i16,i16) = (3,3);
        adjust_node(&mut node_pos, &(1,4), &(2,3));
        assert_eq!(node_pos, (2,4));
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

    #[test]
    fn num_tail_pos2_test() {
        let input = vec![
            "R 5".to_string(),
            "U 8".to_string(),
            "L 8".to_string(),
            "D 3".to_string(),
            "R 17".to_string(),
            "D 10".to_string(),
            "L 25".to_string(),
            "U 20".to_string()
        ];
        assert_eq!(num_tail_positions2(input), 36);

    }

}