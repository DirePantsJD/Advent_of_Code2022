pub fn most_calories(input: Vec<String>) -> u32 {
    let mut most_cals: u32 = 0;
    let mut current_cals_counter: u32 = 0;

    for str_cal in input{
        if str_cal != "" {
            let cal = str_cal.parse::<u32>().unwrap();
            current_cals_counter += cal;
        } else {
            if current_cals_counter > most_cals {
                most_cals = current_cals_counter;
            }
        current_cals_counter = 0;
        }
    }
    if current_cals_counter > most_cals {
        most_cals = current_cals_counter;
    }
    most_cals
}

pub struct TopThree {
    first: u32,
    second: u32,
    third: u32,
}

impl TopThree {
    pub fn new() -> TopThree {
        TopThree {
            first: 0,
            second: 0,
            third: 0,
        }
    }
    pub fn insert(&mut self, val:u32) {
        let lowest = self.lowest();
        if self.first == lowest {
            self.first = val;
        } else if self.second == lowest {
            self.second = val;
        } else if self.third == lowest {
            self.third = val;
        }
    } 
    
    pub fn lowest(&self) -> u32 {
        if self.first < self.second && self.first < self.third {
            return  self.first;
        } else if self.second < self.first && self.second < self.third { 
            return self.second;
        }
        self.third
    }

    pub fn total(&self) -> u32 {
        self.first + self.second + self.third
    }
}

pub fn top_three_calories_sum(input: Vec<String>) -> u32 {
    let mut top_three = TopThree::new();
    let mut current_cals_counter: u32 = 0;

    for str_cal in input{
        if str_cal != "" {
            let cal = str_cal.parse::<u32>().unwrap();
            current_cals_counter += cal;
        } else {
            if current_cals_counter > top_three.lowest() {
                top_three.insert(current_cals_counter);
            }
        current_cals_counter = 0;
        }
    }
    if current_cals_counter > top_three.lowest() {
        top_three.insert(current_cals_counter);
    }
    top_three.total()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bigger_cals() {
        let input: Vec<String> = vec!["1200".to_string(),"300".to_string(),
                                      "".to_string(),"800".to_string(),"200".to_string(),
                                      "".to_string(),"900".to_string(),"900".to_string()];
        assert_eq!(most_calories(input), 1800)
    }

    #[test]
    fn top_three_cals_sum() {
        let input: Vec<String> = vec!["1200".to_string(),"300".to_string(),
                                      "".to_string(),"800".to_string(),"200".to_string(),
                                      "".to_string(),"900".to_string(),"900".to_string(),
                                      "".to_string(),"900".to_string(),"900".to_string(),
                                      "".to_string(),"100".to_string(),"900".to_string()]; 
        assert_eq!(top_three_calories_sum(input), 1800*2 + 1500)
    }
}