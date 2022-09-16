trait Animal {
    fn make_noise(&mut self);
    fn size(&self) -> Size;
}

#[derive(Clone, Copy)]
enum Size {
    Small,
    Normal,
    Big,
    VeryBig(u32),
}

struct Dog {
    size: Size,
    hunger: i32,
}

impl Dog {
    fn eat<T: Animal>(&mut self, other: T) {
        self.hunger -= match other.size() {
            Size::Small => 1,
            Size::Normal => 2,
            Size::Big => 3,
            Size::VeryBig(size) => size as i32,
        }
    }

    fn bark(&mut self) {
        if self.hunger > 5 {
            return;
        }

        match self.size {
            Size::Small => {
                println!("wuf");
                self.hunger += 1;
            }
            Size::Normal => {
                println!("woof!");
                self.hunger += 2;
            }
            Size::Big => {
                println!("WOOOF!");
                self.hunger += 3;
            }
            Size::VeryBig(size) => {
                println!("WOOOF!");
                self.hunger += 4 + size as i32;
            }
        };
    }
}

impl Animal for Dog {
    fn make_noise(&mut self) {
        self.bark();
    }

    fn size(&self) -> Size {
        self.size
    }
}

fn main() {
    let mut golden_retriever = Dog {
        size: Size::Big,
        hunger: 0,
    };

    for _ in 0..5 {
        golden_retriever.bark();
    }
}
