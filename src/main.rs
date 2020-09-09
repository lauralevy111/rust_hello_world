fn main() {

    let logical bool = true;
    //println!("Hello, world!");

   let long_tuple = (1u8, 2u16, 3u32, 4u64,
                     -1i8, -2i16, -3i32, -4i64,
                     0.1f32, 0.2f64,
                     'a', true);
    let l_tuple = (1u8, 2u16, 3u32, 4u64,
                                       -1i8, -2i16, -3i32, -4i64,
                                       0.1f32, 0.2f64,
                                       'a', true);


   println!("long tuple first value: {}", long_tuple.0);


   // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));




}
