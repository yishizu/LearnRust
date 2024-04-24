use hello::greet;

fn main() {
    let a = 10;
    let b = 5;
    let area = area_of(a, b);
    let x =3;
    let y = 24;
    let z = 34;
    let volume = volume(x, y, z);
    println!("Area is {}. Volume is {}",area, volume);
    let tupple:(i32, i32) = (2,6);
    let arr1:[f32;3] = [0.3,0.5,0.7];
    println!("{},{}",tupple.0,tupple.1);
    println!("{},{}",arr1[0],arr1[1]);
    greet();
}

fn area_of(x:i32, y:i32) -> i32{
    return x*y;
}

fn volume(x:i32, y:i32, z:i32) -> i32{
    return  x*y*z;
}