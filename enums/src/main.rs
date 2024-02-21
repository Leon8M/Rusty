enum IPVars {
    V4(u8, u8, u8, u8),
    V6
}
enum Message{
    Quit,
    Move{x:u32, y:u32},
    Write(String),
    Change(i32, i32, i32),
}
impl Message {
    
}
struct Ip{
    which: IPVars,
    address: String,
}
fn main() {
    let local = IPVars::V4(127, 0, 0, 1);
}
fn route(ip: IPVars){}
