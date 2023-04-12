#[cfg(test)]

use super::*;

#[test]
fn statistics_it_works() {
    let result = 2 + 2;

    Statistics::generate_random_vector(10, (5, 10));

    assert_eq!(result, 4);
}