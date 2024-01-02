use utilities::import;
use itertools::Itertools;

fn main() {
    case_1();
}

fn case_1(){
    let input = import::get_input("inputs/case_1.txt");
    println!("input=\n{}", input);
    let num_of_visible_trees = calculate_visible_trees(&input);
    println!("num_of_visible_trees={}", num_of_visible_trees)
}
fn base_case() {
    let base_case_input = import::get_input("inputs/base_case.txt");
    println!("base_case_input=\n{}", base_case_input);
    let base_case_num_of_visible_trees = calculate_visible_trees(&base_case_input);
    println!("base_case_num_of_visible_trees={}", base_case_num_of_visible_trees)
}

fn is_tree_on_edge(x_coordinate: &usize, y_coordinate: &usize, row_length: usize, column_length: usize) -> bool {
    let is_first_row = *y_coordinate == 0;
    let is_last_row = *y_coordinate == column_length - 1;
    let is_first_column = *x_coordinate == 0;
    let is_last_column = *x_coordinate == row_length - 1;
    return is_first_column || is_last_column || is_first_row || is_last_row
}

fn calculate_visible_trees(input: &str) -> usize {
    let matrix_data = parse_to_matrix_of_numbers(input);
    let column_length = matrix_data.len();
    let row_length = matrix_data[0].len();
    // println!("column_length={}", column_length);
    // println!("row_length={}", row_length);
    let x_coordinates: Vec<usize> = (0..row_length).collect();
    let y_coordinates: Vec<usize> = (0..column_length).collect();
    let visible_tree_number = x_coordinates
        .into_iter()
        .cartesian_product(y_coordinates)
        .fold(0, |accum, coordinate| {
            let x = coordinate.0;
            let y = coordinate.1;
            println!("(x,y)=({}, {}) - accum={}", &x, &y, accum);
            let is_edge= is_tree_on_edge(&x, &y, row_length, column_length);
            // println!("is_edge={}", is_edge);
            if is_edge {
                let next_sum = accum + 1;
                return next_sum
            }
            // println!("matrix_data={:?}", matrix_data);
            let row_data = &matrix_data[y];
            let is_left_visible = is_tree_visible_from_left(&x, row_data);
            // println!("is_left_visible={}", is_left_visible);
            if  is_left_visible {
                let next_sum = accum + 1;
                return next_sum
            }
            let is_right_visible = is_tree_visible_from_right(&x, row_data);
            // println!("is_right_visible={}", is_right_visible);
            if is_right_visible {
                let next_sum = accum + 1;
                return next_sum
            }
            let column_data = get_column_data(&x, &matrix_data);
            let is_top_visible = is_tree_visible_from_top(&y, &column_data);
            // println!("is_top_visible={}", is_top_visible);
            if is_top_visible {
                let next_sum = accum + 1;
                return next_sum
            }

            let is_bottom_visible = is_tree_visible_from_bottom(&y, &column_data);
            // println!("is_bottom_visible={}", is_bottom_visible);
            if is_bottom_visible {
                let next_sum = accum + 1;
                return next_sum
            }
            return accum
        });
    return visible_tree_number
}

fn is_tree_visible_from_bottom(y_coordinate: &usize, column_data: &Vec<usize>) -> bool {
    return is_tree_visible_from_right(y_coordinate, column_data)
}

fn is_tree_visible_from_right(x_coordinate: &usize, row_data: &Vec<usize>) -> bool {
    if *x_coordinate == row_data.len() {
        return true
    } else {
        let element_at_x_height: &usize = row_data.get(*x_coordinate).unwrap();
        let left_and_right = row_data.split_at(*x_coordinate + 1);
        let right = left_and_right.1;
        let is_visible = right.iter().all(|height| height < element_at_x_height);
        return is_visible
    }
}

fn is_tree_visible_from_top(y_coordinate: &usize, column_data: &Vec<usize>) -> bool {
   return is_tree_visible_from_left(y_coordinate, column_data);
}

// TODO update logic to see if slice to left contains element that is equal or larger
fn is_tree_visible_from_left(x_coordinate: &usize, row_data: &Vec<usize>) -> bool {
    let element_at_x_height: &usize = row_data.get(*x_coordinate).unwrap();
    let left_and_right = row_data.split_at(*x_coordinate);
    let left = left_and_right.0;
    let is_visible = left.iter().all(|height| height < element_at_x_height);
    return is_visible
}

fn get_column_data(x_coordinate: &usize, matrix_data: &Vec<Vec<usize>>) -> Vec<usize> {
    let column_length = matrix_data.len();
    let column_data =(0..column_length)
        .map(|y| {
            let row = &matrix_data[y];
            row[*x_coordinate]
        }).collect();
    return column_data
}

fn parse_to_matrix_of_numbers(input: &str) -> Vec<Vec<usize>> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let result = lines
        .iter()
        .map(|line| {
            let line_as_numbers = line
                .chars()
                .map(|c| {
                    let number = c.to_digit(10).unwrap() as usize;
                    return number
                })
                .collect::<Vec<usize>>();
            return line_as_numbers
        }).collect();
    return result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_case() {
        let base_case_input = import::get_input("inputs/base_case.txt");
        let actual = calculate_visible_trees(&base_case_input);
        let expected = 21;
        assert_eq!(actual, expected)
    }

    #[test]
    fn is_first_row_edge_case() {
        let actual = is_tree_on_edge(&1, &0, 3, 3);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_last_row_edge_case() {
        let actual = is_tree_on_edge(&1, &2, 3, 3);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_first_column_edge_case() {
        let actual = is_tree_on_edge(&0, &1, 3, 3);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_last_column_edge_case() {
        let actual = is_tree_on_edge(&2, &1, 3, 3);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_not_edge_case() {
        let actual = is_tree_on_edge(&1, &1, 3, 3);
        assert_eq!(actual, false);
    }

    #[test]
    fn is_tree_visible_from_left_process_full_row() {
        let row_data: Vec<usize> = Vec::from([1,2,3,4]);
        let actual = is_tree_visible_from_left(&3, &row_data);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_tree_visible_from_left_process_partial_row() {
        let row_data: Vec<usize> = Vec::from([1,2,4,3]);
        let actual = is_tree_visible_from_left(&2, &row_data);
        assert_eq!(actual, true);
    }

    #[test]
    fn tree_is_not_visible_from_left() {
        let row_data: Vec<usize> = Vec::from([6,5,3,3,2]);
        let actual = is_tree_visible_from_left(&1, &row_data);
        assert_eq!(actual, false);
    }

    #[test]
    fn is_tree_visible_from_top_process_full_row() {
        let row_data: Vec<usize> = Vec::from([1,2,3,4]);
        let actual = is_tree_visible_from_top(&3, &row_data);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_tree_visible_from_top_process_partial_row() {
        let row_data: Vec<usize> = Vec::from([1,2,4,3]);
        let actual = is_tree_visible_from_top(&2, &row_data);
        assert_eq!(actual, true);
    }

    #[test]
    fn tree_is_not_visible_from_top() {
        let row_data: Vec<usize> = Vec::from([1,5,4,3]);
        let actual = is_tree_visible_from_top(&2, &row_data);
        assert_eq!(actual, false);
    }

    #[test]
    fn is_tree_visible_from_right_process_full_row() {
        let row_data: Vec<usize> = Vec::from([4,3,2,1]);
        let actual = is_tree_visible_from_right(&0, &row_data);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_tree_visible_from_right_process_partial_row() {
        let row_data: Vec<usize> = Vec::from([4,4,2,1]);
        let actual = is_tree_visible_from_right(&1, &row_data);
        assert_eq!(actual, true);
    }

    #[test]
    fn tree_is_not_visible_from_right() {
        let row_data: Vec<usize> = Vec::from([4,4,5,1]);
        let actual = is_tree_visible_from_right(&1, &row_data);
        assert_eq!(actual, false);
    }

    #[test]
    fn is_tree_visible_from_bottom_process_full_row() {
        let row_data: Vec<usize> = Vec::from([4,3,2,1]);
        let actual = is_tree_visible_from_bottom(&0, &row_data);
        assert_eq!(actual, true);
    }

    #[test]
    fn is_tree_visible_from_bottom_process_partial_row() {
        let row_data: Vec<usize> = Vec::from([4,4,2,1]);
        let actual = is_tree_visible_from_bottom(&1, &row_data);
        assert_eq!(actual, true);
    }

    #[test]
    fn tree_is_not_visible_from_bottom() {
        let row_data: Vec<usize> = Vec::from([4,4,5,1]);
        let actual = is_tree_visible_from_bottom(&1, &row_data);
        assert_eq!(actual, false);
    }

}
