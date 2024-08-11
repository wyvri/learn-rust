fn main() {
    struct Cat {
        name: String,
        age: u32,
        breed: String,
        sex: char,
    }

    let cat1 = Cat {
        name: String::from("Pixie"),
        age: 6,
        breed: String::from("Diluted Calico"),
        sex: 'F',
    };

    println!("{}, {}, {}, {}", cat1.name, cat1.age, cat1.breed, cat1.sex);

    fn build_cat(name: String, age: u32, breed: String, sex: char) -> Cat {
        Cat {
            name,
            age,
            breed,
            sex,
        }
    }

    let cat2 = build_cat(String::from("Odin"), 6, String::from("Orange Tabby"), 'M');
    println!("{}, {}, {}, {}", cat2.name, cat2.age, cat2.breed, cat2.sex);
}
