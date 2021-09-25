#![feature(impl_trait_in_bindings)]
#![allow(incomplete_features)]


pub mod composite {

    pub fn function<T>(f: impl Fn(T) -> T, g: impl Fn(T) -> T) -> impl Fn(T) -> T {
        //! y = f(g(x))
        move |x: T| { f(g(x))}
    }
}

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
}

pub mod calculus {
     pub mod derivate {

         pub fn vsin(theta: f32) -> f32 {
             theta.sin()
         }

         pub fn vcos(theta: f32) -> f32 {
             -theta.sin()
         }

         pub fn cvsin(theta: f32) -> f32 {
             -theta.cos()
         }

         pub fn cvcos(theta: f32) -> f32 {
             theta.cos()
         }

         pub fn hvsin(theta: f32) -> f32 {
             theta.sin() / 2.0
         }

         pub fn hcvsin(theta: f32) -> f32 {
             -theta.cos() / 2.0
         }

         pub fn hvcos(theta: f32) -> f32 {
             -theta.sin() / 2.0
         }

         pub fn hcvcos(theta: f32) -> f32 {
             theta.cos() / 2.0
         }
    }

    pub mod integral {}
}
