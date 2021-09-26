#![feature(impl_trait_in_bindings)]
#![allow(incomplete_features)]
#[cfg(test)]

mod test {
    use core::f32;
	use mathplus::function;
	use mathplus::trigonometry::*;
	use mathplus::calculus::{derivate, integral};

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
        assert_eq!(vsin(f32::consts::PI), 2.0);
    }

    #[test]
    pub fn test_vcos() {
        assert_eq!(vcos(0.0), 2.0);
        assert_eq!(vcos(f32::consts::PI), 0.0);
    }

    #[test]
    pub fn test_cvsin() {
        assert_eq!(cvsin(0.0), 1.0);
        assert_eq!(cvsin(f32::consts::PI).round(), 1.0);
    }

    #[test]
    pub fn test_cvcos() {
        assert_eq!(cvcos(0.0), 1.0);
        assert_eq!(cvcos(f32::consts::PI).round(), 1.0);
    }

    #[test]
    pub fn test_hvsin() {
        assert_eq!(hvsin(0.0), 0.0);
        assert_eq!(hvsin(f32::consts::PI), 1.0);
    }

    #[test]
    pub fn test_hvcos() {
        assert_eq!(hvcos(0.0), 1.0);
        assert_eq!(hvcos(f32::consts::PI), 0.0);
    }

    #[test]
    pub fn test_hcvsin() {
        assert_eq!(hcvsin(0.0), 0.5);
        assert_eq!(hcvsin(f32::consts::PI / 2.0), 0.0);
    }

    #[test]
    pub fn test_hcvcos() {
        assert_eq!(hcvcos(0.0), 0.5);
        assert_eq!(hcvcos(f32::consts::PI / 2.0), 1.0);
    }

    #[test]
    pub fn test_derivate_vsin() {
        assert_eq!(derivate::vsin(0.0), 0.0);
        assert_eq!(derivate::vsin(f32::consts::PI).round(), 0.0);
    }

    #[test]
    pub fn test_derivate_vcos() {
        assert_eq!(derivate::vcos(0.0), 0.0);
        assert_eq!(derivate::vcos(f32::consts::PI).round(), 0.0);
    }

    #[test]
    pub fn test_derivate_cvsin() {
        assert_eq!(derivate::cvsin(0.0), -1.0);
        assert_eq!(derivate::cvsin(f32::consts::PI), 1.0);
    }

    #[test]
    pub fn test_derivate_cvcos() {
        assert_eq!(derivate::cvcos(0.0), 1.0);
        assert_eq!(derivate::cvcos(f32::consts::PI), -1.0);
    }

    #[test]
    pub fn test_derivate_hvsin() {
        assert_eq!(derivate::hvsin(0.0).round(), 0.0);
        assert_eq!(derivate::hvsin(f32::consts::PI).round(), 0.0);
    }

    #[test]
    pub fn test_derivate_hvcos() {
        assert_eq!(derivate::hvcos(0.0).round(), 0.0);
        assert_eq!(derivate::hvcos(f32::consts::PI).round(), 0.0);
    }

    #[test]
    pub fn test_derivate_hcvsin() {
        assert_eq!(derivate::hcvsin(0.0), -0.5);
        assert_eq!(derivate::hcvsin(f32::consts::PI).round(), 1.0);
    }

    #[test]
    pub fn test_derivate_hcvcos() {
        assert_eq!(derivate::hcvcos(0.0), 0.5);
        assert_eq!(derivate::hcvcos(f32::consts::PI), -0.5);
    }

    #[test]
    pub fn test_integral_vsin() {
        assert_eq!(integral::vsin(0.0), 0.0);
        assert_eq!(integral::vsin(f32::consts::PI), (f32::consts::PI));
    }

    #[test]
    pub fn test_integral_vcos() {
        assert_eq!(integral::vcos(0.0), 0.0);
        assert_eq!(integral::vcos(f32::consts::PI), f32::consts::PI);
    }

    #[test]
    pub fn test_integral_cvsin() {
        assert_eq!(integral::cvsin(0.0), 1.0);
        assert_eq!(integral::cvsin(f32::consts::PI), f32::consts::PI + (-1.0));
    }

    #[test]
    pub fn test_integral_cvcos() {
        assert_eq!(integral::cvcos(0.0), -1.0);
        assert_eq!(integral::cvcos(f32::consts::PI), f32::consts::PI - (-1.0));
    }

    #[test]
    pub fn test_integral_hvsin() {
        assert_eq!(integral::hvsin(0.0), 0.0);
        assert_eq!(integral::hvsin(f32::consts::PI), f32::consts::PI / 2.0);
    }

    #[test]
    pub fn test_integral_hvcos() {
        assert_eq!(integral::hvcos(0.0), 0.0);
        assert_eq!(integral::hvcos(f32::consts::PI), f32::consts::PI / 2.0);
    }

    #[test]
    pub fn test_integral_hcvsin() {
        assert_eq!(integral::hcvsin(0.0), 0.5);
        assert_eq!(integral::hcvsin(f32::consts::PI), (f32::consts::PI + (-1.0)) / 2.0);
    }

    #[test]
    pub fn test_integral_hcvcos() {
        assert_eq!(integral::hcvcos(0.0), -0.5);
        assert_eq!(integral::hcvcos(f32::consts::PI), (f32::consts::PI - (-1.0)) / 2.0);
    }
}