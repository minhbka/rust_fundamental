///////////////////////////////////////////
// BAI 3
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng 
///////////////////////////////////////////


fn main() {
    let a = A {p: Some("p".to_string())};
    a.a();
}

struct A {
    p: Option<String>
}


impl A {
    fn a(&self) -> Self {
        Self::b(&self.p.clone().unwrap());
        Self{p:(self.p.clone())}
    }
    fn b(b: &str) {
        print!("b: {}", b)
    }
}
