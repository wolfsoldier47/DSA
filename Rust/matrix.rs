fn main() {

    let rows = 3;
    let cols = 4;
    let mut matrix: Vec<Vec<i32>> = vec![vec![0;cols];rows];
    matrix[1][2] = 42;

    for row in &matrix{
        println!("{:?}",row)
    }

    // fixed size matrix

    let matrix: [[i32;3];2] = [
      [1,2,3],
      [4,5,6],
    ];
    println!("{}",matrix[0][1]);

}