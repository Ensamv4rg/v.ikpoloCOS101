Struct rectangle{
    width:u32, height:u32
}

impl rectangle{
    fn area(&self)->u32{
        self.width * self.height
    }
}

fn main() {
    let small = rectangl{
        width:10,
        height:20
    };

    println!("width is {} \n height is {} \n area of the rectangle is {}", small.width, small.height, small.area());
}
