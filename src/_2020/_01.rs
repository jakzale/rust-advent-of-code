/// Day 01

use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Report {
    expenses: Vec<i32>,
    lookup: HashMap<i32, HashSet<usize>>
}

impl Report {

    pub fn new() -> Report {
        Report {
            expenses: Vec::new(),
            lookup: HashMap::new()
        }
    }

    pub fn from_vector(expenses: &Vec<i32>) -> Report {
        let mut report = Report::new();

        for expense in expenses {
            report.add_expense(*expense);
        }

        return report;
    }

    pub fn add_expense(&mut self, expense: i32) {
        let expense_id = self.expenses.len();

        self.expenses.push(expense);

        if !self.lookup.contains_key(&expense) {
            self.lookup.insert(expense, HashSet::new());
        }

        self.lookup.get_mut(&expense).unwrap().insert(expense_id);
    }


    pub fn find_and_multiply(&self) -> i32 {
        for i in 0..self.expenses.len() {
            let a = self.expenses[i];
            let b = 2020 - a;

            if let Some(s) = self.lookup.get(&b) {
                for j in s {
                    if i != *j {
                        return a * b;
                    }
                }
            }
        }

        // in case of not found
        return -1;
    }
}

/// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn report_spec() {
        let input = vec![1, 2, 3];
        // immutable report
        let report = Report::from_vector(&input);
        assert_eq!(report.expenses.len(), 3);
        assert_eq!(report.lookup.len(), 3);
        // assert_eq!(format!("{:?}", report), "Report { expenses: [1, 2, 3] }");

        // mutable report
        let mut report = Report::from_vector(&input);
        report.add_expense(4);
        assert_eq!(report.expenses.len(), 4);
        assert_eq!(report.lookup.len(), 4);
        // assert_eq!(format!("{:?}", report), "Report { expenses: [1, 2, 3, 4] }");
    }

    #[test]
    fn example() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let report = Report::from_vector(&input);
        assert_eq!(report.find_and_multiply(), 514579);
    }
}