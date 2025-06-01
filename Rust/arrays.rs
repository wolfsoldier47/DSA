


fn analyze_array(array: &[i32]){
    for i in array.iter(){

        println!("{}",i)
    }
}

fn modify(array: &mut [i32]){
    let mut value = 1;
    for i in array.iter_mut() {
        *i += value;
        value = value + 1;
    }
    println!("{:?}",array);
}

fn main() {
    // fixed side array
    let array1: [i32;5] = [1,2,3,4,5];
    analyze_array(&array1);

    //edditing an array iniated with same value
    let mut array2: [i32;500] = [0;500];
    modify(&mut array2);

    println!("{:?}",array2);

    // Rust slices (on heap)
    let a = [10,20,30,40];
    analyze_array(&a);
    
    //Rust vectors
    let mut vect: Vec<i32> = Vec::new();

    vect.push(1);
    vect.push(2);
    vect.push(3);
    vect.push(4);

    //rust vector predefined using macro
    let mut vect2 = vec![7,8,9,10];
    vect2.pop();
    analyze_array(&vect2);

}