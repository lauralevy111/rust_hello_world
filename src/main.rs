
use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}


struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
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

/*

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    //if  you dont use a variable, rust wants you to prefix it w/ "_"...
    //what is rust
    let ys: [i32; 500] = [0; 500];
    let zs: [i32; 3]= [1, 2, 3];

//if i dont use this string in the println, it doesnt print right bc its a formatting identifier?
//what is rust?

    //printing xs metadata
    println!("first element of the x array: {}", xs[0]);
    println!("second element of the x array: {}", xs[1]);
    //printing yx metadata
    println!("first element of the y array: {}", ys[0]);
    println!("first element of the y array: {}", ys[1]);
    //printing zx metadata
    println!("first element of the z array: {}", zs[0]);
    println!("first element of the z array: {}", zs[1]);

    //using .len method
    println!("array size: {}", xs.len());

    // Arrays are stack allocated
    println!("x array occupies {} bytes", mem::size_of_val(&xs));

    println!("y array occupies {} bytes", mem::size_of_val(&ys));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);
    */






}
