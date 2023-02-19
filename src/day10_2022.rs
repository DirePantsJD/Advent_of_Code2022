use std::vec;

pub fn sum_signal_strengths(input: Vec<String>) -> i32 {
    let mut sum = 0;
    let mut clock: i32 = 1;
    let mut register: i32 = 1;
    for line in input {
        if clock > 220 {
            break;
        }
        match line.as_str() {
            "noop" => clock += 1,
            _ => {
                clock += 1;
                if (clock - 20) % 40 == 0 {
                    sum += clock * register;
                }
                clock += 1;
                let num: i32 = line.split(" ").nth(1).unwrap().parse().unwrap();
                register += num;
            },
        }
        if (clock - 20) % 40 == 0 {
            sum += clock * register;
        }
    }
    sum
}

pub fn draw_letters(input: Vec<String>) {
    let mut screen: Vec<char> = vec![];
    let mut draw_pos: i32 = 0;
    let mut sprite_pos: i32 = 1;
    for line in input {
        match line.as_str() {
            "noop" => {
                if draw_pos == 40 {
                    draw_pos = 0;
                }
                dbg!(draw_pos);
                dbg!(sprite_pos);
                if sprite_pos.abs_diff(draw_pos) <= 1  {
                    screen.push('#');
                } else {
                    screen.push('.');
                }
                draw_pos += 1;
            },
            _ => {
                if draw_pos == 40 {
                    draw_pos = 0;
                }
                dbg!(draw_pos);
                dbg!(sprite_pos);
                if sprite_pos.abs_diff(draw_pos) <= 1  {
                    screen.push('#');
                } else {
                    screen.push('.');
                }
                draw_pos += 1;
                if draw_pos == 40 {
                    draw_pos = 0;
                }
                dbg!(draw_pos);
                dbg!(sprite_pos);
                if sprite_pos.abs_diff(draw_pos) <= 1  {
                    screen.push('#');
                } else {
                    screen.push('.');
                }
                draw_pos += 1;
                let num: i32 = line.split(" ").nth(1).unwrap().parse().unwrap();
                sprite_pos = sprite_pos + num;
                
            },
        }
    }
    let mut screen_iter = screen.iter();
    for _ in 0..6 {
        for _ in 0..40{
            print!("{}",screen_iter.next().unwrap());
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_signal_strengths() {
        let input = vec![
            "addx 15".to_string(),
            "addx -11".to_string(),
            "addx 6".to_string(),
            "addx -3".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx -8".to_string(),
            "addx 13".to_string(),
            "addx 4".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx -35".to_string(),
            "addx 1".to_string(),
            "addx 24".to_string(),
            "addx -19".to_string(),
            "addx 1".to_string(),
            "addx 16".to_string(),
            "addx -11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 21".to_string(),
            "addx -15".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -3".to_string(),
            "addx 9".to_string(),
            "addx 1".to_string(),
            "addx -3".to_string(),
            "addx 8".to_string(),
            "addx 1".to_string(),
            "addx 5".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -36".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "addx 7".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "addx 6".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 7".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx -13".to_string(),
            "addx 13".to_string(),
            "addx 7".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "addx -33".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 8".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 2".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 17".to_string(),
            "addx -9".to_string(),
            "addx 1".to_string(),
            "addx 1".to_string(),
            "addx -3".to_string(),
            "addx 11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -13".to_string(),
            "addx -19".to_string(),
            "addx 1".to_string(),
            "addx 3".to_string(),
            "addx 26".to_string(),
            "addx -30".to_string(),
            "addx 12".to_string(),
            "addx -1".to_string(),
            "addx 3".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -9".to_string(),
            "addx 18".to_string(),
            "addx 1".to_string(),
            "addx 2".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 9".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 2".to_string(),
            "addx -37".to_string(),
            "addx 1".to_string(),
            "addx 3".to_string(),
            "noop".to_string(),
            "addx 15".to_string(),
            "addx -21".to_string(),
            "addx 22".to_string(),
            "addx -6".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx -10".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 20".to_string(),
            "addx 1".to_string(),
            "addx 2".to_string(),
            "addx 2".to_string(),
            "addx -6".to_string(),
            "addx -11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string()
        ];
    assert_eq!(sum_signal_strengths(input),13140);
    }

    #[test]
    fn test_draw() {
        let input = vec![
            "addx 15".to_string(),
            "addx -11".to_string(),
            "addx 6".to_string(),
            "addx -3".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx -8".to_string(),
            "addx 13".to_string(),
            "addx 4".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx -35".to_string(),
            "addx 1".to_string(),
            "addx 24".to_string(),
            "addx -19".to_string(),
            "addx 1".to_string(),
            "addx 16".to_string(),
            "addx -11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 21".to_string(),
            "addx -15".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -3".to_string(),
            "addx 9".to_string(),
            "addx 1".to_string(),
            "addx -3".to_string(),
            "addx 8".to_string(),
            "addx 1".to_string(),
            "addx 5".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -36".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "addx 7".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "addx 6".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 7".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx -13".to_string(),
            "addx 13".to_string(),
            "addx 7".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "addx -33".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 8".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 2".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 17".to_string(),
            "addx -9".to_string(),
            "addx 1".to_string(),
            "addx 1".to_string(),
            "addx -3".to_string(),
            "addx 11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -13".to_string(),
            "addx -19".to_string(),
            "addx 1".to_string(),
            "addx 3".to_string(),
            "addx 26".to_string(),
            "addx -30".to_string(),
            "addx 12".to_string(),
            "addx -1".to_string(),
            "addx 3".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -9".to_string(),
            "addx 18".to_string(),
            "addx 1".to_string(),
            "addx 2".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 9".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 2".to_string(),
            "addx -37".to_string(),
            "addx 1".to_string(),
            "addx 3".to_string(),
            "noop".to_string(),
            "addx 15".to_string(),
            "addx -21".to_string(),
            "addx 22".to_string(),
            "addx -6".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx -10".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 20".to_string(),
            "addx 1".to_string(),
            "addx 2".to_string(),
            "addx 2".to_string(),
            "addx -6".to_string(),
            "addx -11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string()
        ];
    draw_letters(input);
    }

}