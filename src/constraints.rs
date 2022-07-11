#[derive(Debug)]
pub struct Constraints {
    size: usize,
    constraints: Vec<Vec<u32>>,
}

impl Constraints {
    pub fn new(size: usize) -> Constraints {
        let mut constraints = Vec::new();
        constraints.resize(size, Vec::new());

        Constraints { size, constraints }
    }

    pub fn set(&mut self, index: usize, constraints: Vec<u32>) -> Result<(), String> {
        if index < self.size {
            self.constraints[index] = constraints;
            Ok(())
        } else {
            Err("Index out of bounds for contraints".into())
        }
    }

    pub fn get(&self, index: usize) -> Result<Vec<u32>, String> {
        if index < self.size {
            Ok(self.constraints[index].clone())
        } else {
            Err("Index out of bounds for contraints".into())
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    // Returns the number of constraints for the longest row / column
    // And its index
    pub fn get_max_len(&self) -> (usize, usize) {
        let (mut max_len, mut max_i) = (0, 0);

        for (i, cs) in self.constraints.iter().enumerate() {
            if cs.len() > max_len {
                max_len = cs.len();
                max_i = i;
            }
        }

        (max_len, max_i)
    }

    // Returns the number of digits for the longest row / column
    // And its index
    pub fn get_max_digits(&self) -> (usize, usize) {
        let (mut max_digits, mut max_i) = (0, 0);

        for (i, cs) in self.constraints.iter().enumerate() {
            let mut curr_digits = 0;
            for c in cs.iter() {
                curr_digits += c.to_string().len();
            }

            if curr_digits > max_digits {
                max_digits = curr_digits;
                max_i = i;
            }
        }

        (max_digits, max_i)
    }

    pub fn get_len_at(&self, index: usize) -> usize {
        self.constraints[index].len()
    }

    pub fn get_digits_at(&self, index: usize) -> usize {
        let mut digits = 0;
        for c in &self.constraints[index] {
            digits += c.to_string().len();
        }

        digits
    }
}
