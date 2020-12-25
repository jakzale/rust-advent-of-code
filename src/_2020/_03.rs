use std::collections::HashSet;

#[derive(Debug)]
struct Row {
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
    pub fn from_str(grid: &str) -> Self {
        let rows: Vec<Row> = grid.split_whitespace().map(|x| Row::from_str(x)).collect();
        Grid {
            rows,
        }
    }
    pub fn height(self: &Self) -> usize {
        return self.rows.len();
    }

    pub fn is_tree(self: &Self, row: usize, column: usize) -> bool {
        if row >= self.height() {
            return false;
        }

        return self.rows[row].is_tree(column);
    }

    pub fn count_trees(self: &Self, row_step: usize, col_step: usize) -> usize {
        let row_iter = (0..self.height()).step_by(row_step);
        let col_iter = (0..).step_by(col_step);

        row_iter
            .zip(col_iter)
            .skip(1)
            .map(|(r,c)| self.is_tree(r,c))
            .map(|t| if t { 1 } else { 0 })
            .sum()
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

    #[test]
    fn empty_grid() {
        let grid = Grid::from_str("");

        assert_eq!(0, grid.height());
        assert_eq!(false, grid.is_tree(0, 0));
        assert_eq!(false, grid.is_tree(1, 3));
        assert_eq!(false, grid.is_tree(2, 6));
        assert_eq!(false, grid.is_tree(3, 9));
    }

    #[test]
    fn sample_grid() {
        let input =
            r#"..##.......
               #...#...#..
               .#....#..#.
               ..#.#...#.#
               .#...##..#.
               ..#.##.....
               .#.#.#....#
               .#........#
               #.##...#...
               #...##....#
               .#..#...#.#"#;
        let grid = Grid::from_str(input);

        assert_eq!(11, grid.height());
        assert_eq!(false, grid.is_tree(1,  3));
        assert_eq!(true,  grid.is_tree(2,  6));
        assert_eq!(false, grid.is_tree(3,  9));
        assert_eq!(true,  grid.is_tree(4,  12));
        assert_eq!(true,  grid.is_tree(5,  15));
        assert_eq!(false, grid.is_tree(6,  18));
        assert_eq!(true,  grid.is_tree(7,  21));
        assert_eq!(true,  grid.is_tree(8,  24));
        assert_eq!(true,  grid.is_tree(9,  27));
        assert_eq!(true,  grid.is_tree(10, 30));

        assert_eq!(7, grid.count_trees(1, 3));
    }
}
