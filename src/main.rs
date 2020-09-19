#[derive(Debug)]

//use std::mem;

// This function borrows a slice
//fn analyze_slice(slice: &[i32]) {
//    println!("first element of the slice: {}", slice[0]);
//    println!("the slice has {} elements", slice.len());
//}


/*
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);


// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}
*/

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
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
    //struct practice:
    /*

    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    //// Instantiate a unit struct
    //let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(_integer, _decimal) = pair;

    let area = calculate_area(rectangle);
    println!("area is {:?}",area);
    */

}

fn calculate_area(rectangle: Rectangle)-> f32{
        let y = rectangle.top_left.y - rectangle.bottom_right.y;
        let x = rectangle.bottom_right.x - rectangle.top_left.x;

        let area = x*y;
        return area;

}
