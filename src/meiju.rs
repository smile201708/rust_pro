enum Animal {
    Cat,
    Dog,
    Bird,
}

impl Animal {
    fn sound(&self) {
        match self {
            Animal::Cat => println!("Cat!"),
            Animal::Dog => println!("Dog!"),
            Animal::Bird => println!("Bird!"),
        }
    }
}

fn main() {
    let animals: Vec<Animal> = vec![Animal::Cat, Animal::Dog, Animal::Bird];
    for animal in animals {
        animal.sound();
    }
}
