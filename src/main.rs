
use num_traits::{Float,Num};
use std::ops::{Add, AddAssign, Sub,SubAssign, Mul, MulAssign,Div,DivAssign,Neg};
#[allow(dead_code)]
// Define a generic Complex structure
#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex<T> {
   real: T,
   imag: T,
}

// Implement the + trait for Complex<T>
impl<T> Add for Complex<T>
where
   T: Add<Output = T>,
{
   type Output = Self;

   fn add(self, other: Self) -> Self {
      Self {
         real: self.real + other.real,
         imag: self.imag + other.imag,
      }
   }
}

// Implement the += trait for Complex<T>
impl<T> AddAssign for Complex<T>
where
   T: AddAssign,
{
   fn add_assign(&mut self, other: Self) {
      self.real += other.real;
      self.imag += other.imag;
   }
}
// Implement the - trait for Complex<T>
impl<T> Sub for Complex<T>
where
   T: Sub<Output = T>,
{
   type Output = Self;

   fn sub(self, other: Self) -> Self {
      Self {
         real: self.real - other.real,
         imag: self.imag - other.imag,
      }
   }
}

// Implement the -= trait for Complex<T>
impl<T> SubAssign for Complex<T>
where
   T: SubAssign,
{
   fn sub_assign(&mut self, other: Self) {
      self.real -= other.real;
      self.imag -= other.imag;
   }
}

// Implement * (a + bi) * (c + di) = (ac - bd) + (ad + bc)i
impl<T> Mul for Complex<T>
where
   T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Clone
{
   type Output = Self;

   fn mul(self, other: Self) -> Self {
      Self {
         real: self.real.clone() * other.real.clone() - self.imag.clone() * other.imag.clone(),
         imag: self.real * other.imag + self.imag * other.real,
      }
   }
}

// Implement *= trait for Complex<T>
impl<T> MulAssign for Complex<T>
where
   T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Clone,
{
   fn mul_assign(&mut self, other: Self) {
      let real = self.real.clone() * other.real.clone() - self.imag.clone() * other.imag.clone();
      let imag = self.real.clone() * other.imag + self.imag.clone() * other.real;
      self.real = real;
      self.imag = imag;      
   }
}

// Implement / z1 = a + bi,z2 = c + di then z1 / z2 = [(a * c + b * d) + (b * c - a * d)i] / (c² + d²)
impl<T> Div for Complex<T>
where
   T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T>+ Clone
{
   type Output = Self;

   fn div(self, other: Self) -> Self {
      let denom = other.real.clone() * other.real.clone() + other.imag.clone() * other.imag.clone();
      Self {         
         real: (self.real.clone() * other.real.clone() + self.imag.clone() * other.imag.clone())/denom.clone(),
         imag: (self.imag * other.real - self.real * other.imag)/denom,
      }
   }
}

// Implement *= trait for Complex<T>
impl<T> DivAssign for Complex<T>
where
   T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Div<Output = T>+ Clone
{
   fn div_assign(&mut self, other: Self) {
      let denom = other.real.clone() * other.real.clone() + other.imag.clone() * other.imag.clone();
      let real = (self.real.clone() * other.real.clone() + self.imag.clone() * other.imag.clone())/denom.clone();
      let imag = (self.imag.clone() * other.real - self.real.clone() * other.imag)/denom;
      self.real = real;
      self.imag = imag;      
   }
}

// Implement standard functions for complex numbers
impl<T> Complex<T>
{
   // absolute value/modulus
   fn abs(&self) -> T 
      where
         T: Mul<Output = T> + Add<Output = T> + Clone
   {
      self.real.clone() * self.real.clone() + self.imag.clone() * self.imag.clone()
   }

   // 
   fn real(&self) -> T
      where
         T: Clone    
   {
      self.real.clone()
   }

   fn imag(&self) -> T
      where
         T: Clone        
   {
      self.imag.clone()
   }

   fn conj(&self) -> Self
      where
         T: Neg<Output = T> + Clone,
   {
      Self {
         real: self.real.clone(),
         imag: -self.imag.clone(),
      }
   }

   fn degrees_to_radians(degrees: T) -> T
      where
         T: Float,
   {
      let pi = T::from(std::f64::consts::PI).unwrap(); // Convert PI to type T
      degrees * pi / T::from(180.0).unwrap()
   }

   // Returns a Complex<T> value from polar coords ( angle in radians )
   fn polar(magnitude: T,phase_angle: T ) -> Complex<T>
   where
      T: Float + Num + Clone,
   {
      Complex { real: magnitude * phase_angle.cos(), imag: magnitude * phase_angle.sin()}
   }


}





fn main() {
   // Example usage with f64
   let mut c1 = Complex { real: 1.0, imag: 2.0 };
   let c2 = Complex { real: 3.0, imag: 4.0 };
   
   let c3 = c1 * c2;
   c1 *= c2;
   
   println!("c1: {:?} c3: {:?} abs(c3) {}", c1, c3, c3.abs());

}


// struct Matrix<T> {
//    data: [T],
//    rows: u64,
//    cols: u64,   
// }

// impl<T> Matrix<T> {
// }
