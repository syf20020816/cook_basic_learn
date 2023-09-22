#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person1 = Person {
        name: "Matt".to_string(),
        age: 16,
    };
    let person2 = Person {
        name: "Jay".to_string(),
        age: 18,
    };
    let mut persons = vec![person1, person2];
    if persons[0].age.lt(&persons[1].age) {
        persons.swap(0, 1);
    }
    dbg!(persons);
}
