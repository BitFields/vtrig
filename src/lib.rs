#![feature(impl_trait_in_bindings)]
#![allow(incomplete_features)]


pub mod composite {

    pub fn function<T>(f: impl Fn(T) -> T, g: impl Fn(T) -> T) -> impl Fn(T) -> T {
        //! y = f(g(x))
        move |x: T| { f(g(x))}
    }
}

pub mod trigonometry {

    pub fn vsin(x: f32) -> f32 {
        //! versed sin
        //!
        //! 1 - cos(Θ)
        1.0 - x.cos()
    }
    
    pub fn cvsin(x: f32) -> f32 {
        //! coversed sin
        //!
        //! 1 - sin(Θ)
        1.0 - x.sin()
    }

    pub fn vcos(x: f32) -> f32 {
        //! versed cos
        //!
        //! 1 + cos(Θ)
        1.0 + x.cos()
    }

    pub fn cvcos(x: f32) -> f32 {
        //! coversed cos
        //!
        //! 1 + sin(Θ)
        1.0 + x.sin()
    }
    
    pub fn hvsin(x: f32) -> f32 {
        //! half versed sin
        //!
        //! (1 - cos(Θ)) / 2
        (1.0 - x.cos()) / 2.0
    }
    
    pub fn hcvsin(x: f32) -> f32 {
        //! half coversed sin
        //!
        //! (1 - sin(Θ)) / 2
        (1.0 - x.sin()) / 2.0
    }
    
    pub fn hvcos(x: f32) -> f32 {
        //! half versed cos
        //!
        //! (1 + cos(Θ)) / 2
        (1.0 + x.cos()) / 2.0
    }
    
    pub fn hcvcos(x: f32) -> f32 {
        //! half coversed cos
        //!
        //! (1 + sin(Θ)) / 2
        (1.0 + x.sin()) / 2.0
    }
}

pub mod calculus {
     pub mod derivate {

         pub fn vsin(x: f32) -> f32 {
             x.sin()
         }

         pub fn vcos(x: f32) -> f32 {
             -x.sin()
         }

         pub fn cvsin(x: f32) -> f32 {
             -x.cos()
         }

         pub fn cvcos(x: f32) -> f32 {
             x.cos()
         }

         pub fn hvsin(x: f32) -> f32 {
             x.sin() / 2.0
         }

         pub fn hcvsin(x: f32) -> f32 {
             -x.cos() / 2.0
         }

         pub fn hvcos(x: f32) -> f32 {
             -x.sin() / 2.0
         }

         pub fn hcvcos(x: f32) -> f32 {
             x.cos() / 2.0
         }
    }

    pub mod integral {

        pub fn vsin(x: f32) -> f32 {
            //! ∫ vsin(x) dx = x - sin x + C
            x - x.sin()
        }
        
        pub fn vcos(x: f32) -> f32 {
            //! ∫ vcos(x) dx = x + sin x + C
            x + x.sin()
        }

        pub fn cvsin(x: f32) -> f32 {
            //! ∫ cvsin(x) dx = x + cos x + C
            x + x.cos()
        }

        pub fn cvcos(x: f32) -> f32 {
            //! ∫ cvcos(x) dx = x - cos x + C
            x - x.cos()
        }

        pub fn hvsin(x: f32) -> f32 {
            //! ∫ hvsin(x) dx = (x - sin x) / 2 + C
            (x - x.sin()) / 2.0
        }

        pub fn hcvsin(x: f32) -> f32 {
            //! ∫ hvsin(x) dx = (x + cos x) / 2 + C
            (x + x.cos()) / 2.0
        }

        pub fn hvcos(x: f32) -> f32 {
            //! ∫ hvsin(x) dx = (x + sin x) / 2 + C
            (x + x.sin()) / 2.0
        }
        
        pub fn hcvcos(x: f32) -> f32 {
            //! ∫ hvsin(x) dx = (x - cos x) / 2 + C
            (x - x.cos()) / 2.0
        }
    }
}
