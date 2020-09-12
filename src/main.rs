

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
/*
    let logical bool = true;
    //println!("Hello, world!");

   let long_tuple = (1u8, 2u16, 3u32, 4u64,
                     -1i8, -2i16, -3i32, -4i64,
                     0.1f32, 0.2f64,
                     'a', true);
    let l_tuple2 = (1u8, 2u16, 3u32, 4u64,
                                       -1i8, -2i16, -3i32, -4i64,
                                       0.1f32, 0.2f64,
                                       'a', true);
    let l_tuple3 = (1u8, 2u16, 3u32, 4u64,
                    -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64,
                    'a', true);



   println!("long tuple first value: {}", long_tuple.0);


   // Tuples can be tuple member
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
*/ //commenting out practice code

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    //printing xs metadata
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    //using .len method
    println!("array size: {}", xs.len());



}
