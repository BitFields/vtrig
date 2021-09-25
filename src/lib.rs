#![feature(impl_trait_in_bindings)]
#![allow(incomplete_features)]


pub mod trigonometry {

    pub fn vsin(theta: f32) -> f32 {
        //! versed sin
        //!
        //! 1 - cos(Θ)
        1.0 - theta.cos()
    }
    
    pub fn cvsin(theta: f32) -> f32 {
        //! coversed sin
        //!
        //! 1 - sin(Θ)
        1.0 - theta.sin()
    }

    pub fn vcos(theta: f32) -> f32 {
        //! versed cos
        //!
        //! 1 + cos(Θ)
        1.0 + theta.cos()
    }

    pub fn cvcos(theta: f32) -> f32 {
        //! coversed cos
        //!
        //! 1 + sin(Θ)
        1.0 + theta.sin()
    }
    
    pub fn hvsin(theta: f32) -> f32 {
        //! half versed sin
        //!
        //! (1 - cos(Θ)) / 2
        (1.0 - theta.cos()) / 2.0
    }
    
    pub fn hcvsin(theta: f32) -> f32 {
        //! half coversed sin
        //!
        //! (1 - sin(Θ)) / 2
        (1.0 - theta.sin()) / 2.0
    }
    
    pub fn hvcos(theta: f32) -> f32 {
        //! half versed cos
        //!
        //! (1 + cos(Θ)) / 2
        (1.0 + theta.cos()) / 2.0
    }
    
    pub fn hcvcos(theta: f32) -> f32 {
        //! half coversed cos
        //!
        //! (1 + sin(Θ)) / 2
        (1.0 + theta.sin()) / 2.0
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

pub mod composite {

    pub fn function(f: impl Fn(f32) -> f32, g: impl Fn(f32) -> f32) -> impl Fn(f32) -> f32 {
        //! y = f(g(x))
        move |x: f32| { f(g(x))}
    }

    #[test]
    pub fn test_function() {
        let f: impl Fn(f32) -> f32 = |x: f32| x + 1.0;
        let g: impl Fn(f32) -> f32 = |x: f32| x + 2.0;
        let composite_fn = function(f, g);

        let mut x: f32 = 0.0;
        // (0.0 + 2.0) + 1.0 == 3.0
        assert_eq!(composite_fn(x), 3.0);

        x = 2.0;
        // (2.0 + 2.0) + 1.0 == 5.0
        assert_eq!(composite_fn(x), 5.0);
    }
}