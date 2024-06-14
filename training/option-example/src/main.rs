// fn divider(a: i32, b: i32) {
//     let result = if b == 0 { None } else { Some(a / b) };
//
//     match result {
//         None => println!("Cannot divide by 0"),
//         Some(2) => println!("the result is 2"),
//         Some(x) => println!("result: {x}"),
//     }
// }

fn divider(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }

    let result = a / b;
    return Some(result);
}

fn main() {
    // divider(20, 0);
    // divider(10, 2);
    // divider(30, 2);
    // divider(4, 2);

    // Pattern matching variabel x hanya akan diakses di block Some(x) saja.
    // result akan menghasilkan type Option<i32>
    let result1 = divider(20, 2);
    match result1 {
        None => println!("cannot divide by 0"),
        Some(x) => println!("The result is {x}"),
    }

    // Pattern matching yang menampung hasil ke dalam variabel
    // result akan menghasilkan tipe i32
    // disini match memastikan akan mendapatkan 2 concrete value
    // antara x atau 0
    let result2 = match divider(10, 2) {
        None => {
            println!("cannot divide by 0");
            0
        }
        Some(x) => x,
    };

    println!("{:?}", result2);

    // menggunakan metode unwrap()
    // jika ternyata result3 berisi None, maka method unwrap() diakses
    // akan menyebabkan error panic, karena tidak ada value alias None
    let result3 = divider(10, 2);
    if result3 != None {
        let number = result3.unwrap();
        println!("result3 = {}", number);
    } else {
        let number = result3.unwrap();
        println!("result3 = {}", number);
    }

    // akan mengeksekusi is.none() method
    // let result4 = divider(20, 0);
    // akan mengeksekusi is_some();
    let result4 = divider(20, 10);
    if result4.is_some() {
        let number = result4.unwrap();
        println!("result4 = {}", number);
    }

    if result4.is_none() {
        let number = result4.unwrap();
        println!("result4 = {}", number);
    }

    // menggunakan method unwrap_or_default()
    // mengembalikan nilai T ketika data berisi Some<T>
    // jika ternyata isinya adalah None maka yang akan dikembalikan adalah default dari T
    // default dari T adalah 0
    let result5 = divider(24, 0);
    {
        let number = result5.unwrap_or_default();
        println!("result5 = {}", number);
    }

    // menggunakan method unwrap_or(0)
    // method ini mengembalikan nilai T ketika data berisi None maka
    // akan mengembalikan argumen tersebut dalam hal ini adalah 0
    let result6 = divider(50, 0);
    {
        let number = result6.unwrap_or(0);
        println!("result6 = {}", number);
    }

    // menggunakan method unwrap_or_else();
    // mengembalikan nilai T jika berisi Some<T>
    // jika isi datanya adalah None maka nilai yang dikembalikan adalah eksekusi closure
    let result7 = divider(60, 0);
    {
        let number = result7.unwrap_or_else(|| 0);
        println!("result7 = {}", number);
    }
}
