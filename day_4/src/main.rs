const TARGET: &str = "XMAS";

fn read_input(path: &str)->String{
    return std::fs::read_to_string(path).expect("FileNotFound");
}

fn create_char_matrix(content: &String) -> Vec<Vec<char>>{
    let mut vec: Vec<Vec<char>> = Vec::new();
    let lines: Vec<&str> = content.split("\r\n").collect();
    for line in lines{
        vec.push(line.chars().collect());
    }
    return vec;
}

fn is_target(str:String) -> bool{
    return str == TARGET;
}

fn check_upper_triangle_diagonal(matrix:&Vec<Vec<char>>, position: (usize,usize)) -> bool{
    let row = position.0;
    let column = position.1;
    let n =matrix.len(); 
    let mut str: String = String::with_capacity(4);
    let steps = std::cmp::min(std::cmp::min(row+1, n - column), 4);
    for i in  0..steps{
        str.push(matrix[row - i][column + i]);
    }
    return is_target(str);
}

fn check_bottom_triangle_diagonal(matrix:&Vec<Vec<char>>, position: (usize,usize)) -> bool{
    let row = position.0;
    let n =matrix.len(); 
    let column = position.1;
    let mut str: String = String::with_capacity(4);
    let steps = std::cmp::min(std::cmp::min(n - row, column+1), 4);
    for i in  0..steps{
        str.push(matrix[row+i][column-i]);
    }
    return is_target(str);
}

fn check_backward_diagonal(matrix:&Vec<Vec<char>>, position: (usize,usize)) -> bool{
    let row = position.0;
    let column = position.1;
    let mut str: String = String::with_capacity(4);

    let steps = std::cmp::min(std::cmp::min(row, column+1), 4);
    for i in  0..steps{
        str.push(matrix[row-i][column-i]);
    }
    return is_target(str);
}

fn check_forward_diagonal(matrix:&Vec<Vec<char>>, position: (usize,usize)) -> bool {
    let row = position.0;
    let column = position.1;
    let n = matrix.len();
    let mut str: String = String::with_capacity(4);
    let steps = std::cmp::min(std::cmp::min(n - row, n - column), 4);
    for i in  0..steps{
        str.push(matrix[row+i][column+i]);
    }
    return is_target(str);
}

fn check_down(matrix:&Vec<Vec<char>>, position: (usize,usize)) -> bool{
    let row = position.0;
    let column = position.1;
    let mut str: String = String::with_capacity(4);
    let n = matrix.len();
    let steps = std::cmp::min(n - row, 4);
    for i in  0..steps{
        str.push(matrix[row+i][column]);
    }
    return is_target(str);
}

fn check_up(matrix:&Vec<Vec<char>>, position: (usize,usize)) -> bool{
    let row = position.0;
    let column = position.1;
    let mut str: String = String::with_capacity(4);
    let steps = std::cmp::min(row + 1, 4);
    for i in  0..steps{
        str.push(matrix[row-i][column]);
    }
    return is_target(str);
}

fn check_backwards(matrix:&Vec<Vec<char>>, position: (usize,usize)) -> bool{
    let row = position.0;
    let column = position.1;
    let mut str: String = String::with_capacity(4);
    let steps = std::cmp::min(column + 1, 4);
    for i in  0..steps{
        str.push(matrix[row][column - i]);
    }
    return is_target(str);
}

fn check_forwards(matrix:&Vec<Vec<char>>, position: (usize,usize)) -> bool{
    let row = position.0;
    let column = position.1;
    let n = matrix.len();
    let mut str: String = String::with_capacity(4);
    let steps = std::cmp::min(n - column, 4);
    for i in  0..steps{
        str.push(matrix[row][column + i]);
    }
    return is_target(str);
}

fn count_xmas_in_matrix(matrix: Vec<Vec<char>>) -> u32{
    let dim = matrix.len();
    let mut count: u32 = 0;
    for i in 0..dim{
        for j in 0..dim{
            let c = matrix[i][j];
            let position = (i,j);
            if c == 'X' {
                count += check_upper_triangle_diagonal(&matrix, position) as u32;
                count += check_backwards(&matrix, position) as u32;
                count += check_up(&matrix, position) as u32;
                count += check_down(&matrix, position) as u32;
                count += check_forward_diagonal(&matrix, position) as u32;
                count += check_backward_diagonal(&matrix, position) as u32;
                count += check_bottom_triangle_diagonal(&matrix, position) as u32;
                count += check_forwards(&matrix, position) as u32;

            }
        }
    }

    return count;
}




fn main() {
    let input = read_input("input.txt");
    let matrix = create_char_matrix(&input);
    let count = count_xmas_in_matrix(matrix);
    println!("xmas count: {count}");
}
