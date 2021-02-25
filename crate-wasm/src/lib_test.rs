use super::*;

#[cfg(test)]
mod lib_test {
    const MAX_ITERATIONS: u32 = 1000;
    const IMAGE_SIZE: usize = 256 * 256 * 4;
    #[test]
    fn get_escape_time_if_not_in_set_escapes() {
        let escapes_iterations_top_left =
            super::get_escape_time(-2.0, 1.0, MAX_ITERATIONS, 3.0, 2).0;
        assert_ne!(escapes_iterations_top_left, MAX_ITERATIONS);

        let escapes_iterations_center_right =
            super::get_escape_time(1.0, 0.0, MAX_ITERATIONS, 3.0, 2).0;
        assert_ne!(escapes_iterations_center_right, MAX_ITERATIONS);
    }

    #[test]
    fn get_escape_time_if_in_set_stays_bounded() {
        let bounded_iterations_origin = super::get_escape_time(0.0, 0.0, MAX_ITERATIONS, 3.0, 2).0;
        assert_eq!(bounded_iterations_origin, MAX_ITERATIONS);

        let bounded_iterations_bulb = super::get_escape_time(-1.0, 0.0, MAX_ITERATIONS, 3.0, 2).0;
        assert_eq!(bounded_iterations_bulb, MAX_ITERATIONS);
    }

    #[test]
    fn generate_image_outputs_correct_length() {
        let image = super::generate_image(0.0, 0.0, 2.0, MAX_ITERATIONS, 2);
        assert_eq!(image.len(), IMAGE_SIZE);

        let zoomed_image = super::generate_image(8476.0, 9507.0, 12.0, MAX_ITERATIONS, 2);
        assert_eq!(zoomed_image.len(), IMAGE_SIZE);
    }

    #[test]
    fn generate_image_outputs_valid_colors() {
        let image = super::generate_image(0.0, 0.0, 2.0, MAX_ITERATIONS, 2);
        for n in image.clone().iter_mut() {
            assert!(n >= &mut 0 && n <= &mut 255);
        }

        let zoomed_image = super::generate_image(8476.0, 9507.0, 12.0, MAX_ITERATIONS, 2);
        for n in zoomed_image.clone().iter_mut() {
            assert!(n >= &mut 0 && n <= &mut 255);
        }
    }
}
