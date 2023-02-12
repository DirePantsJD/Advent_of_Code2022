use std::cmp::max as max;

pub fn visible_trees(input: Vec<String>) -> u32 {
    let n_rows = input.len();
    let n_cols = input[0].len();
    let mut result: u32 = (n_cols*2 + n_rows*2 - 4) as u32;

    let trees = input
        .iter()
        .map(|str| str.chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>();

    for row in 1..(n_rows - 1) {
        for col in 1..(n_cols - 1) {
            let (left,mid_right) = trees[row * n_cols..(row+1) * n_cols]
                .split_at(col);
            let right = &mid_right[1..mid_right.len()];
            let mut counter = n_cols - (col + 1);
            let mut aux_vec = trees.clone();

            aux_vec.retain(|_| {counter+=1; counter % n_cols == 0 });

            let (up,mid_down)= aux_vec[..].split_at(row);
            let down = &mid_down[1..mid_down.len()];

            result += check_tree(trees[row * n_cols + col], left, right, up, down);
        }
    }
    result
}

fn check_tree(tree: char, left: &[char], right: &[char], up: &[char], down: &[char]) -> u32 {
    let mut vec: Vec<bool> = vec![];
    let mut value = true;
    for other_tree in left {
        if *other_tree >= tree {
            value = false;
        }
    }
    vec.push(value);
    value = true;
    for other_tree in right {
        if *other_tree >= tree {
            value = false;
        }
    }
    vec.push(value);
    value = true;
    for other_tree in up {
        if *other_tree >= tree{
            value = false;
        }
    }
    vec.push(value);
    value = true;
    for other_tree in down {
        if *other_tree >= tree {
            value = false;
        }
    }
    vec.push(value);
    if vec.iter().fold(false, |acc,val| acc || *val) {
        1
    } else {
        0
    }
}

pub fn max_scenic_score(input: Vec<String>) -> u32 {
    let n_rows = input.len();
    let n_cols = input[0].len();
    let mut result: u32 = 0;

    let trees = input
        .iter()
        .map(|str| str.chars().collect::<Vec<char>>())
        .flatten()
        .collect::<Vec<char>>();

    for row in 1..(n_rows - 1) {
        for col in 1..(n_cols - 1) {
            let mut trees1 = trees.clone();
            let (left,mid_right) = trees1[row * n_cols..(row+1) * n_cols]
                .split_at_mut(col);
            let right = &mid_right[1..mid_right.len()];
            let mut counter = n_cols - (col + 1);

            let mut trees2 = trees.clone();
            trees2.retain(|_| {counter+=1; counter % n_cols == 0 });

            let (up,mid_down)= trees2[..]
                .split_at_mut(row);
            
            let down = &mid_down[1..mid_down.len()];
            left.reverse();
            up.reverse();
            result = max(result,check_score(trees[row * n_cols + col], left, right, up, down))
        }
    }
    result
}

fn check_score(tree: char,left: &[char],right: &[char],up: &[char],down: &[char]) -> u32 {
    let mut left_score = 0;
    let mut right_score = 0;
    let mut up_score = 0;
    let mut down_score = 0;

    for other_tree in left {
        if *other_tree >= tree {
            left_score += 1;
            break;
        }
        left_score += 1;
    }
    for other_tree in right {
        if *other_tree >= tree {
            right_score += 1;
            break;
        }
        right_score += 1;
    }
    for other_tree in up {
        if *other_tree >= tree{
            up_score += 1;
            break;
        }
        up_score += 1;
    }
    for other_tree in down {
        if *other_tree >= tree {
            down_score += 1;
            break;
        }
        down_score += 1;
    }
    left_score * right_score * up_score * down_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let vec = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];
        assert_eq!(visible_trees(vec),21);
    }

    #[test]
    fn test_max_score() {
        let vec = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];
        assert_eq!(max_scenic_score(vec),8);
    }

    #[test]
    fn test_score() {
        assert_eq!(check_score('5', &['3','3'], &['4','9'], &['3','5','3'], &['3']),8)
    }
}