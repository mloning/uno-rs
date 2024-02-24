struct Cycle {
    values: Vec<usize>,
    current: usize,
    is_reversed: bool,
}

impl Iterator for Cycle {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.values.len() as i8;

        // update current
        println!("before {:?}", self.current);
        let next = match self.is_reversed {
            true => self.current as i8 - 1,
            false => self.current as i8 + 1,
        };
        println!("next {:?}", next);
        // println!("n {:?}", n);
        let current = next.rem_euclid(n);
        println!("after {:?}", current);
        self.current = current as usize;

        // select item
        let item = self.values[self.current];
        println!("item {:?}", item);
        Some(item)
    }
}

impl Cycle {
    fn new(n: usize) -> Self {
        let range = 0..n;
        let values = range.collect();
        Cycle {
            values,
            current: n - 1,
            is_reversed: false,
        }
    }

    fn reverse(&mut self) {
        println!("reverse");
        self.is_reversed = match self.is_reversed {
            true => false,
            false => true,
        }
    }
}

// fn main() {
//     let mut cycle = cycle();
//
//     println!("> {:?}", cycle.next());
//     println!("> {:?}", cycle.next());
//     println!("> {:?}", cycle.next());
//     println!("> {:?}", cycle.next());
// }

#[cfg(test)]
mod tests {
    use super::*; // bring private functions into scope

    #[test]
    fn test_cycle_iter_values_next_reverse_next_3() {
        let mut cycle = Cycle::new(3);
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 2);
        cycle.reverse();
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 2);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 0);
        cycle.reverse();
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 2);
    }

    #[test]
    fn test_cycle_iter_values_next_reverse_next_5() {
        let mut cycle = Cycle::new(5);
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 2);
        cycle.reverse();
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 4);
        assert_eq!(cycle.next().unwrap(), 3);
        assert_eq!(cycle.next().unwrap(), 2);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 0);
    }

    // TODO
    // #[test]
    // fn test_cycle_iter_values_init_reverse() {}

    #[test]
    fn test_cycle_iter_values_double_reverse() {
        let mut cycle = Cycle::new(3);
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 2);
        cycle.reverse();
        cycle.reverse();
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 2);
    }

    #[test]
    fn test_cycle_iter_values_next() {
        let mut cycle = Cycle::new(3);
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 2);
        assert_eq!(cycle.next().unwrap(), 0);
        assert_eq!(cycle.next().unwrap(), 1);
        assert_eq!(cycle.next().unwrap(), 2);
    }
}
