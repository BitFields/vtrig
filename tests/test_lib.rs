#![feature(impl_trait_in_bindings)]
#![allow(incomplete_features)]
#[cfg(test)]

mod test {
	use mathplus::trigonometry::*;
	use mathplus::function;

	#[test]
    pub fn test_composite() {
        let f: impl Fn(f32) -> f32 = |x: f32| x + 1.0;
        let g: impl Fn(f32) -> f32 = |x: f32| x + 2.0;
        let composite_fn = function::composite(f, g);

        let mut x: f32 = 0.0;
        // (0.0 + 2.0) + 1.0 == 3.0
        assert_eq!(composite_fn(x), 3.0);

        x = 2.0;
        // (2.0 + 2.0) + 1.0 == 5.0
        assert_eq!(composite_fn(x), 5.0);
    }

	#[test]
    pub fn test_vsin() {
        assert_eq!(vsin(0.0), 0.0);
    }

    #[test]
    pub fn test_cvsin() {
        assert_eq!(cvsin(0.0), 1.0);
    }

    #[test]
    pub fn test_vcos() {
        assert_eq!(vcos(0.0), 2.0);
    }

    #[test]
    pub fn test_cvcos() {
        assert_eq!(cvcos(0.0), 1.0);
    }

    #[test]
    pub fn test_hvsin() {
        assert_eq!(hvsin(0.0), 0.0);
    }

     #[test]
    pub fn test_hcvsin() {
        assert_eq!(hcvsin(0.0), 0.5);
    }

    #[test]
    pub fn test_hvcos() {
        assert_eq!(hvcos(0.0), 1.0);
    }

    #[test]
    pub fn test_hcvcos() {
        assert_eq!(hcvcos(0.0), 0.5);
    }
}