mod front_of_house {
    // front_of_house 是 eat_at_restaurant 的兄弟，因此可以存取，不需加 pub。
    pub mod hosting {
        // 必須要是 pub 才能被 eat_at_restaurant() 存取
        pub fn add_to_waitlist() {
            // 必須要是 pub 才能被 eat_at_restaurant() 存取
            println!("add_to_waitlist()");
        }

        fn seat_at_table() {
            println!("seat_at_table()");
        }
    }

    mod serving {
        fn take_order() {
            println!("take_order()");
        }

        fn serve_order() {
            println!("serve_order()");
        }

        fn take_payment() {
            println!("take_payment()");
        }
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    //-------- pub struct 的範例
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I like {} for toast.", meal.toast);
    // println!("The fruit is {}.", meal.seasonal_fruit);

    //-------- pub enum 的範例
    let _order1 = back_of_house::Appetizer::Salad;
    let _order2 = back_of_house::Appetizer::Soup;

    // 因為有 use，所以現在 hosting 也在這個 scope 中了
    hosting::add_to_waitlist();
}

fn deliver_order() {
    println!("deliver_order()");
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
        // super 指的是 back_of_house 的上一層，也就是 root crate
    }

    pub fn cook_order() {
        println!("cook_order()");
    }

    //-------- pub struct 的範例
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Breakfast {
                toast: toast.to_owned(),
                seasonal_fruit: "peaches".to_owned(),
            }
        }
    }

    //-------- pub enum 的範例
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod customer {
    // 因為 crate root 已經有 hosting 了（見第 30 行），因此可以直接 use
    // super (i.e. crate root) 的 hosting
    use super::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

// 把 front_of_house::hosting::* 和 back_of_house::* 都拉進新的 module 中
pub mod facilities {
    pub use super::back_of_house::*;
    pub use super::front_of_house::hosting::*;
}
