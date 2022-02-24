#[derive(Debug)]
struct App {
    rest: i32,
    text: String,
}

impl App {
    fn new() -> Self {
        Self {
            rest: 0, text: "hello".to_string()
        }
    }

    // Self is the type https://doc.rust-lang.org/reference/paths.html#self-1
    fn update(&mut self) {
        let Self{rest, text} = self;
        *rest += 1;
    }
}

#[test]
fn create() {
    let mut app = App::new();
    app.update();
    println!("{:?}", app)
}