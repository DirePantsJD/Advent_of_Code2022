struct Move(u8,usize,usize);

#[derive(Debug,PartialEq)]
struct Stacks {
    stacks: Vec<String>
}

impl Stacks {
    pub fn new() -> Stacks {
        Stacks { stacks: vec!["BQC".to_string(),
                              "RQZW".to_string(),
                              "BMRLV".to_string(),
                              "CZHVTW".to_string(),
                              "DZHBNVG".to_string(),
                              "HNPCJFVQ".to_string(), 
                              "DGTRWZS".to_string(),
                              "CGMNBWZP".to_string(),
                              "NJBMWQFP".to_string()
                             ]
                }
    }

    pub fn move_crate(&mut self,crate_move: Move) {
        for _ in 0..crate_move.0 {
            let cr4te = self.stacks[crate_move.1-1].pop().unwrap();
            self.stacks[crate_move.2-1].push(cr4te);
        }
    }
    
    pub fn get_top_of_each(&mut self) -> String {
        let mut top = String::new();
        for stack in &mut self.stacks {
            let top_crate = (*stack).pop().unwrap();
            top.push(top_crate);
        }
        top
    }
}

pub fn top_of_stacks(input: Vec<String>) -> String {
    let mut stacks = Stacks::new();
    for move_command in input {
        let mut aux_iter = move_command.split(" ");
        let qnt: u8 = aux_iter.nth(1).unwrap().parse().unwrap();
        let from: usize = aux_iter.nth(1).unwrap().parse().unwrap();
        let to: usize = aux_iter.nth(1).unwrap().parse().unwrap();
        let crate_move = Move(qnt,from,to);
        stacks.move_crate(crate_move);
    } 
    stacks.get_top_of_each()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_crate_teste() {
        let mut stacks_before = Stacks::new();
        let stacks_after = Stacks{ 
                    stacks: vec!["".to_string(),
                                 "RQZW".to_string(),
                                 "BMRLVCQB".to_string(),
                                 "CZHVTW".to_string(),
                                 "DZHBNVG".to_string(),
                                 "HNPCJFVQ".to_string(), 
                                 "DGTRWZS".to_string(),
                                 "CGMNBWZP".to_string(),
                                 "NJBMWQFP".to_string()
                                ]};
        let move_cmd = Move(3,1,3);
        stacks_before.move_crate(move_cmd);
        assert_eq!(stacks_before, stacks_after);
    }
}