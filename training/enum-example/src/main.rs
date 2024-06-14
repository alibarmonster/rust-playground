mod constants;
enum Food {
    PenyetanTerangBulan,
    PizzaNanas,
    EsKrimIkanMujaer,
    MiGorengKuah,
    MakananLainnya(String),
    MieSetan {
        level_pedas: i32,
        pakek_piring: bool,
    },
}

fn main() {
    // let makanan_favorit: Food = Food::PenyetanTerangBulan;

    // let nasi_goreng = String::from("Nasi Goreng");
    // let makanan_favorit: Food = Food::MakananLainnya(nasi_goreng);

    let makanan_favorit: Food = Food::MieSetan {
        level_pedas: 22,
        pakek_piring: false,
    };

    match makanan_favorit {
        Food::PenyetanTerangBulan => {
            println!("aku suka sekali terang bulaan");
        }

        Food::PizzaNanas => {
            println!("aku suka sekali Pizza Nanas");
        }

        Food::EsKrimIkanMujaer => {
            println!("aku suka sekali EsKrimIkanMujaer");
        }

        Food::MiGorengKuah => {
            println!("aku pengen Mi Goreng Kuah");
        }

        Food::MakananLainnya(m) => {
            println!("kamu suka {m} juga? sama aku juga begitu");
        }

        Food::MieSetan {
            level_pedas,
            pakek_piring,
        } => {
            if level_pedas > 20 {
                println!("mie setan level {level_pedas} is too much")
            } else {
                println!("mie setan level {level_pedas} is perfect")
            };

            if !pakek_piring {
                println!("serius lu bro, gapake piring?");
            }
        }
    }
    println!("-------------------------------------------------------------------");

    let company = constants::Company::Apple;

    match company {
        constants::Company::Apple => {
            print!("Apple");
        }
        _ => {
            print!("other than Apple");
        }
    }
}
