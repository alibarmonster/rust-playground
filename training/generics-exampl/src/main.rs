struct Point<T, U> {
    x: T,
    y: T,
    z: U,
}

impl<T, U> Point<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }

    fn get_z(&self) -> &U {
        &self.z
    }
}

fn main() {
    string_x_times("alibar monster", 10);

    let data_arr = [1, 3, 5, 6, 7];
    let largest_number = find_largest_number(&data_arr);
    println!("largest number from data_arr = {:?}", largest_number);

    let smallest_number = find_smallest_number(&data_arr);
    println!("smalles number from data_arr = {:?}", smallest_number);

    let numbers = vec![10, 20, 30, 40, 2, 3, 0, 99, 100];
    print_largest_or_smallest_number(&numbers);

    let num_one: Point<i32, f64> = Point {
        x: 32,
        y: 250,
        z: 21.5,
    };

    println!(
        "x: {}, y: {}, z: {}",
        num_one.get_x(),
        num_one.get_y(),
        num_one.get_z()
    );
}

fn string_x_times<T: std::fmt::Debug>(data: T, x: i32) {
    for i in 0..x {
        println!("{:?}, {}", data, i);
    }
}

fn find_largest_number<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn find_smallest_number<T: std::cmp::PartialOrd>(list_number: &[T]) -> &T {
    let mut smallest = &list_number[0];
    for item in list_number {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

// Traits definition #1
// fn print_largest_or_smallest_number<T: std::cmp::PartialOrd + std::fmt::Debug>(list_number: &[T]) {
//     let largest = find_largest_number(list_number);
//     let smallest = find_smallest_number(list_number);
//     println!("largest number is : {:?}", largest);
//     println!("smallest number is: {:?}", smallest);
//     println!("total data is {:?}", list_number.len());
// }

// equals with traits definition #1 using where
fn print_largest_or_smallest_number<T>(list_number: &[T])
where
    T: std::cmp::PartialOrd + std::fmt::Debug,
{
    let largest = find_largest_number(list_number);
    let smallest = find_smallest_number(list_number);
    println!("largest number is : {:?}", largest);
    println!("smallest number is : {:?}", smallest);
    println!("total data is {:?}", list_number.len());
}
