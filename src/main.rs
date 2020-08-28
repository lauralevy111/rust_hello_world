fn main() {

    let logical bool = true;
    //println!("Hello, world!");

   let long_tuple = (1u8, 2u16, 3u32, 4u64,
                     -1i8, -2i16, -3i32, -4i64,
                     0.1f32, 0.2f64,
                     'a', true);


   println!("long tuple first value: {}", long_tuple.0);
   println!("long tuple second value: {}", long_tuple.1);

}
