trait Animal {
    fn sound(&self);
}

struct Cat;
struct Dog;
struct Bird;

impl Animal for Cat {
    fn sound(&self) {
        println!("Cat!");
    }
}

impl Animal for Dog {
    fn sound(&self) {
        println!("Dog!");
    }
}

impl Animal for Bird {
    fn sound(&self) {
        println!("bird!");
    }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Cat),
        Box::new(Dog),
        Box::new(Bird),
    ];
    for animal in animals {
        animal.sound();
    }
}
