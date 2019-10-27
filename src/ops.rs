use std::f64;
use std::iter::repeat;

// TODO make a trait to use with num and big numbers

/// Calculates the euclidean distance for two f64 vectors
///
/// The Euclidean distance between two vectors is
/// the square root of the sum of the power of the distance
/// between the components of the two vectors.
///
/// If the vectors have different lenghts the missing component is set to 0.
/// The distance gets absolutized by raising to the power of 2,
/// thus the euclidean distance is always a positive number.
///
/// # Examples
///
/// ```
/// use rustlearningmachine as rlm;
///
///  assert_eq!(rlm::ops::euclidean_distance(vec!(7., 4., 2., 1.), vec!(2., 7., 1.)), 6.);
/// ```
pub fn euclidean_distance(a: Vec<f64>, b: Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    let matrix = if a.len() >= b.len() {
        a.iter().zip(b.iter().chain(repeat(&0.)))
    } else {
        b.iter().zip(a.iter().chain(repeat(&0.)))
    };
    for (el_a, el_b) in matrix {
        sum += (*el_a - *el_b).powi(2);
    }
    sum.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance_same_lenght() {
        assert_eq!(euclidean_distance(vec!(7.), vec!(2.)), 5.);
    }
    #[test]
    fn test_euclidean_distance_different_lenghts() {
        assert_eq!(
            euclidean_distance(vec!(3., 7., 7., 8., 2., 1.), vec!(2., 2., 14., 7.)),
            9.
        );
    }
}
