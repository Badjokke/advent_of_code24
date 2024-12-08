const TARGET: &str = "XMAS";
const TARGET2: &str = "MAS";
const TARGET2_REVERSED: &str = "SAM";
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


fn check_forward_diagonal_2(matrix:&Vec<Vec<char>>, position: (usize,usize)) -> bool {
    let row = position.0;
    let column = position.1;
    let n = matrix.len();
    let mut str: String = String::with_capacity(3);
    let steps = std::cmp::min(std::cmp::min(n - row, n - column), 3);
    for i in  0..steps{
        str.push(matrix[row+i][column+i]);
    }
    return is_target_2(&str);
}

fn check_backward_diagonal_2(matrix:&Vec<Vec<char>>, position: (usize,usize)) -> bool{
    let row = position.0;
    let column = position.1;
    let mut str: String = String::with_capacity(3);

    let steps = 3;//std::cmp::min(3);
    for i in  0..steps{
        str.push(matrix[row-i][column+i]);
    }
    return is_target_2(&str);
}

fn is_target_2(str: &str) -> bool {
    return TARGET2 == str || TARGET2_REVERSED == str;
}

fn count_mas_in_matrix(matrix: Vec<Vec<char>>) -> u32{
    let dim = matrix.len();
    let mut count: u32 = 0;
    for i in 0..dim - 2{
        for j in 0..dim{
            let c = matrix[i][j];
            let position = (i,j);
            if c == 'M' || c == 'S'{
                let position2 = (position.0+2, position.1);
                if check_forward_diagonal_2(&matrix, position) && check_backward_diagonal_2(&matrix, position2) {
                    count += 1;
                }
            }
        } 
    }
    return count;
}

fn main() {
    let input = read_input("input.txt");
    let matrix = create_char_matrix(&input);
    let count = count_mas_in_matrix(matrix);
    println!("xmas count: {count}");
}
