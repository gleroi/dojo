struct Base {
    a: u32,
}

impl Base {
    fn content(&self) {
        println!("Base.content: {}", self.a);
    }

    fn a(&self) {
        println!("Base.a: {}", self.a);
    }
}

struct Child {
    base: Base,
    b: f32,
}

impl std::ops::Deref for Child {
    type Target = Base;

    fn deref(&self) -> &Base {
        &self.base
    }

}

impl Child {
    fn content(&self) {
        println!("Child.content: {} {}", self.a, self.b);
    }
}

fn main() {
    let b = Child { b: 1.0, base: Base { a: 5 } };
    b.content();
    b.a();
}
