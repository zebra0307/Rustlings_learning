// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

pub mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    use self::fruits::PEAR   ;
    use self::veggies::CUCUMBER ;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }

    pub const FRUIT: &str = PEAR;
    pub const VEGGIE: &str = CUCUMBER;
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::FRUIT,
        delicious_snacks::VEGGIE,
    );
}
