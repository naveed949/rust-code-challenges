/// An enum representing the temperature scale.
#[derive(Debug, PartialEq)]
pub enum Scale {
    Celsius,
    Fahrenheit,
}

/// A struct representing a temperature value.
pub struct Temperature {
    pub degrees: f32,
    pub scale: Scale,
}

impl Temperature {
    /// Creates a new `Temperature` instance with the given degrees in Celsius.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_code_challenges::temprature::{Temperature, Scale};
    /// 
    /// let temp = Temperature::new(20.0);
    /// assert_eq!(temp.degrees, 20.0);
    /// assert_eq!(temp.scale, Scale::Celsius);
    /// ```
    pub fn new(degrees: f32) -> Self {
        Temperature {
            degrees,
            scale: Scale::Celsius,
        }
    }

    /// Converts the temperature value to Celsius.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_code_challenges::temprature::{Temperature, Scale};
    /// 
    /// let temp = Temperature::new(20.0);
    /// assert_eq!(temp.to_celsius(), 20.0);
    /// ```
    pub fn to_celsius(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees,
            Scale::Fahrenheit => (self.degrees - 32.0) * (5.0 / 9.0),
        }
    }

    /// Converts the temperature value to Fahrenheit.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_code_challenges::temprature::{Temperature, Scale};
    /// 
    /// let temp = Temperature::new(20.0);
    /// assert_eq!(temp.to_fahrenheit(), 68.0);
    /// ```
    pub fn to_fahrenheit(&self) -> f32 {
        match self.scale {
            Scale::Celsius => ((9.0 / 5.0) * self.degrees) + 32.0,
            Scale::Fahrenheit => self.degrees,
        }
    }
}

#[cfg(test)]
mod temprature_test {
    use super::*;

    #[test]
    fn one_degree() {
        let cold = Temperature::new(1.0);
        assert!((cold.to_fahrenheit() - 33.8) < 0.01);
        assert!((cold.to_fahrenheit() - 33.8) >= 0.0);
    }

    #[test]
    fn boiling() {
        let hot = Temperature::new(100.0);
        assert!((hot.to_fahrenheit() - 212.0) < 0.01);
        assert!((hot.to_fahrenheit() - 212.0) >= 0.0);
    }

    #[test]
    fn freezing() {
        let freezing = Temperature {
            degrees: Temperature::new(0.0).to_fahrenheit(),
            scale: Scale::Fahrenheit,
        };

        assert!(freezing.to_celsius() < 0.001);
        assert!(freezing.to_celsius() > -0.01);
    }
}
