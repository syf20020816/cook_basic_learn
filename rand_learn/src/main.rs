mod lib;

use lib::{
    build_rand, build_rand_range, build_rand_range_better, build_normal_distribution,
    build_rand_type, build_random_string,build_random_string_bind
};

fn main() {
    // build_rand();
    // build_rand_range();
    // build_rand_range_better();
    // build_normal_distribution();
    // build_rand_type()
    // build_random_string();
    build_random_string_bind();
}
