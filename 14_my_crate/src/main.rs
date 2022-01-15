// use my_crate::kinds::PrimaryColor;
// use my_crate::utils::mix;

use my_crate::mix;
use my_crate::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
