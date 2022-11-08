fn main() {
    // Cargo.toml 中定義的 project name 是 "restaurant-lib"。
    // 但由於 '-' 不是名稱的合法字元，因此當成 crate name 使用時，就會變成 '_'。
    restaurant_lib::eat_at_restaurant();

    println!("(Back to Main)");

    // 因為在 src/lib.rs 中有 `pub use crate::front_of_house::hosting`，
    // 所以我們可以使用 `restaurant_lib::hosting` 呼叫到 add_to_waitlist();
    restaurant_lib::hosting::add_to_waitlist();

    // 但直接使用完整路徑反而不行，因為 front_of_house 不是 pub
    //restaurant_lib::front_of_house::hosting::add_to_waitlist();

    // 利用 re-export 可以讓架構從外部看起來不一樣
    restaurant_lib::facilities::add_to_waitlist();
    restaurant_lib::facilities::cook_order();
}
