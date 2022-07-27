pub enum FoodType {
    SeaFood,
    LandFood,
    AirFood,
}

impl PartialEq for FoodType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
