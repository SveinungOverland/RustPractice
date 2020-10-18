struct Test {
    one: Box<i32>,
    two: Box<i32>,
    three: Box<i32>,
}

impl Test {
    fn new() -> Test {
        Test {
            one: Box::new(1),
            two: Box::new(2),
            three: Box::new(3),
        }
    }
    fn change_one_to_five(&mut self) {
        *self.one.as_mut() = 5
    }

    fn result(&self) -> i32 {
        self.one.as_ref() * self.two.as_ref() * self.three.as_ref()
    }
}

fn main() {
    println!("Hello, world!");
    let mut temp = Test::new();
    temp.change_one_to_five();
    println!("{}", temp.result());
}
