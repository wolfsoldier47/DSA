


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
}