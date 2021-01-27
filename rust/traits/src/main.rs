trait Animal {
    fn sound(&self) -> String;
}

fn get_sound<T: Animal>(animal: T) -> String {
    animal.sound()
}

struct Dog {}
struct Cat {}

impl Animal for Dog {
    fn sound(&self) -> String {
        String::from("Au au")
    }
}

impl Animal for Cat {
    fn sound(&self) -> String {
        String::from("Miau")
    }
}

fn main() {
    let dog = Dog {};
    let cat = Cat {};
    println!("Sound from Dog: {}", get_sound(dog));
    println!("Sound from Cat: {}", get_sound(cat));
}