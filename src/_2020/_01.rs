/// Day 01

use std::collections::HashSet;

#[derive(Debug)]
pub struct Report {
    expenses: Vec<i32>,
}

// Going for simple brute force search
impl Report {
    pub fn new() -> Report {
        Report {
            expenses: Vec::new()
        }
    }

    pub fn from_vector(expenses: Vec<i32>) -> Report {
        Report {
            expenses
        }
    }

    pub fn add_expense(&mut self, expense: i32) {
        self.expenses.push(expense);
    }

    fn find_expenses_recur(&self, remaining_sum: i32, remaining_count: usize, used_indices: HashSet<usize>) -> Vec<i32> {
        // adding optimization
        if remaining_count <= 0 || remaining_sum <= 0 || used_indices.len() == self.expenses.len() {
            return Vec::new();
        }

        let indices_to_check = (0..self.expenses.len()).filter(|x| !used_indices.contains(x));

        for i in indices_to_check {
            let a = self.expenses[i];
            let rest_remaining_sum = remaining_sum - a;
            let rest_remaining_count = remaining_count - 1;
            let mut rest_used_indices = used_indices.clone();
            rest_used_indices.insert(i);

            let mut candidate = vec![a];
            let mut rest = self.find_expenses_recur(rest_remaining_sum, rest_remaining_count, rest_used_indices);
            candidate.append(&mut rest);

            if candidate.len() == remaining_count && candidate.iter().sum::<i32>() == remaining_sum {
                return candidate;
            }
        }

        return Vec::new();
    }

   fn find_expenses(&self, expense_sum: i32, element_count: usize) -> Vec<i32> {
        self.find_expenses_recur(expense_sum, element_count, HashSet::new())
    }

    pub fn part_one(&self) -> i32 {
        let expenses = self.find_expenses(2020, 2);
        if expenses.is_empty() {
            return -1;
        } else {
            return expenses.iter().product();
        }
    }

    pub fn part_two(&self) -> i32 {
        let expenses = self.find_expenses(2020, 3);
        if expenses.is_empty() {
            return -1;
        } else {
            return expenses.iter().product();
        }
    }

}

/// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn report_spec() {
        let input = vec![1, 2, 3];
        // empty report
        let report = Report::new();
        assert_eq!(format!("{:?}", report), "Report { expenses: [] }");
        // immutable report
        let report = Report::from_vector(input.clone());
        assert_eq!(report.expenses.len(), 3);
        assert_eq!(format!("{:?}", report), "Report { expenses: [1, 2, 3] }");

        // mutable report
        let mut report = Report::from_vector(input);
        report.add_expense(4);
        assert_eq!(report.expenses.len(), 4);
        assert_eq!(format!("{:?}", report), "Report { expenses: [1, 2, 3, 4] }");
    }

    #[test]
    fn example() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let report = Report::from_vector(input);
        assert_eq!(report.part_one(), 514579);
    }
}