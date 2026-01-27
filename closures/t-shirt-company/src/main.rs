#[derive(Debug, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_prof1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_prof1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_prof1, giveaway1
    );

    let user_prof2 = Some(ShirtColor::Blue);
    let giveaway2 = store.giveaway(user_prof2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_prof2, giveaway2
    );
}
