mod _2019 {
    pub mod _01 {
        #[derive(Debug)]
        pub struct Module {
            mass: u32
        }        

        impl Module {
            pub fn new(mass: u32) -> Module {
                Module {
                    mass
                }
            }

            fn single_fuel(mass: u32) -> u32 {
                let mass_ = mass / 3;
                if mass_ > 2 {
                    return mass_ - 2;
                } else {
                    return 0;
                }

            }

            pub fn fuel(&self) -> u32 {
                Module::single_fuel(self.mass)
            }

            pub fn total_fuel(&self) -> u32 {
                let mut remaining_mass = self.mass;
                let mut total_fuel: u32 = 0;

                while remaining_mass > 0 {
                    let f = Module::single_fuel(remaining_mass);
                    total_fuel += f;
                    remaining_mass = f;
                }

                return total_fuel;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::_2019::_01::Module;

    #[test]
    fn module_spec() {
        let m = Module::new(5);
        assert_eq!(format!("{:?}", m), "Module { mass: 5 }");
    }

    #[test]
    fn module_fuel_spec() {
        assert_eq!(Module::new(12).fuel(), 2);
        assert_eq!(Module::new(14).fuel(), 2);
        assert_eq!(Module::new(1969).fuel(), 654);
        assert_eq!(Module::new(100756).fuel(), 33583);
    }

    #[test]
    fn module_total_fuel_spec() {
        assert_eq!(Module::new(12).total_fuel(), 2);
        assert_eq!(Module::new(14).total_fuel(), 2);
        assert_eq!(Module::new(1969).total_fuel(), 966);
        assert_eq!(Module::new(100756).total_fuel(), 50346);
    }
}