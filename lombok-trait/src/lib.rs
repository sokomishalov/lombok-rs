// this trait was introduced to prevent overloading compile issue
// in case of using it with AllArgsContructor
pub trait NoArgsContructor {
    fn new() -> Self;
}
