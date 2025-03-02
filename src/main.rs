//Practice with slices - string and array
fn main () {
    let s: String = String::from("Mighty Mouse");//Strings
    let t = &s;//full reference of s, no ownership transferred
    let v = &s[0..6];//reference of s. bytes 0 - 5 = "Mighty"
    let z = &s[7..];//reference of s. bytes 7 - end = "Mouse"
    println!("{}", s);
    println!("{}", t);
    println!("{}", v);
    println!("{}", z);
    
    let mut cereals = [//Arrays
    String::from("Cookie Crisp"),
    String::from("Cinnamon Toash Crunch"),
    String::from("Frosted Flakes"),
    String::from("Cocoa Puffs"),
    String::from("Captain Crunch"),
];
let first_two = &cereals[..2];
println!("{:?}", first_two);

let mid_three = &cereals[1..4];
println!("{:?}", mid_three);

let last_three = &mut cereals[2..];//changed to mut here so I could change out element below
println!("{:?}", last_three);

last_three[2] = String::from("Lucky Charms");//targeting last element and switching out "Captain Crunch" for "Lucky Charms"
println!("{:?}", cereals);//able to access original array and "Capatain Crunch" has been replaced with "Lucky Charms"

let cookie = &cereals[0];
let cookie = &cookie[..6];//variable shadowning, cookie became bytes 0-5 in "Cookie Crisp", "Cookie"
println!("{:?}", cookie);

let cocoa_puffs = &cereals[3];//variable declaration, cocoa_puffs = "Cocoa Puffs"
let puffs = &cocoa_puffs[6..];//reference of cocoa_puffs, puffs = 6th byte till the end = "Puffs"
println!("{:?}", puffs);
}
