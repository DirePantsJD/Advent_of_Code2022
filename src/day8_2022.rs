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
            // dbg!(aux_vec.clone());
            let (up,mid_down)= aux_vec[..].split_at(row);
            let down = &mid_down[1..mid_down.len()];
            result += check_tree(trees[row * n_cols + col], left, right, up, down);
        }
    }
    result
}

fn check_tree(tree: char, left: &[char], right: &[char], up: &[char], down: &[char]) -> u32 {
    //dbg!(tree);
    //dbg!(left);
    //dbg!(right);
    //dbg!(up);
    //dbg!(down);
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
}