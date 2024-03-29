#[derive(Debug)]
#[allow(dead_code)]
enum Species {
    Apple,
    Orange,
    Bannana,
}

#[derive(Debug)]
struct Fruit {
    species: Species,
    name: String,
    color: (i32, i32, i32),
}

pub fn demo() {
    println!("Running structure demo!");
    let apple = Fruit {
        species: Species::Apple,
        name: "Red Apple".to_string(),
        color: (255, 0, 0),
    };

    println!("My fruit, {} is a {:?} and is colored {:?}",
             apple.name,
             apple.species,
             apple.color);
    println!("Structure demo done!");
}
