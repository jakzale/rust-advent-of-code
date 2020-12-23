use std::collections::HashSet;

#[derive(Debug)]
pub struct Row {
    width: usize,
    trees: HashSet<usize>,
}

impl Row {
    pub fn from_str(row: &str) -> Self {
        let width : usize = row.len();
        let trees : HashSet<usize> =
            row.chars()
               .enumerate()
               .filter(|x| x.1 == '#')
               .map(|x| x.0)
               .collect();

        Row {
            width,
            trees,
        }
    }

    // Technically his doesn't need to be public
    pub fn width(self: &Self) -> usize {
        return self.width;
    }

    pub fn is_tree(self: &Self, column: usize) -> bool {
        if self.width() == 0 {
            return false;
        }

        let index : usize = column % self.width();
        return self.trees.contains(&index);
    }
}

#[derive(Debug)]
pub struct Grid {
    rows: Vec<Row>,
}

// API
impl Grid {
    pub fn height(self: &Self) -> usize {
        return self.rows.len();
    }

    pub fn is_tree(self: &Self, row: usize, column: usize) -> bool {
        if row >= self.height() {
            return false;
        }

        return self.rows[row].is_tree(column);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_empty_test() {
        let row = Row::from_str("");

        assert_eq!(0, row.width());
        assert_eq!(false, row.is_tree(13));
    }

    #[test]
    fn row_sample_test() {
        let row = Row::from_str(".##.#.#..");

        assert_eq!(9, row.width());
        assert_eq!(false, row.is_tree(0));
        assert_eq!(false, row.is_tree(9));
        assert_eq!(true, row.is_tree(1));
        assert_eq!(true, row.is_tree(10));
    }
}
