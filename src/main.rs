/* Format Macro - Similar to println!, but instead of printing a string, it returns a formatted string, inlcuding any interpolated contents
Can then be assigned to a variable or set as a return value of a function*/


fn main() {
let first_name = String::from("Sylvester");
let last_name = "Stallone";

let icon = format!("{} {}", first_name, last_name);
println!("{icon}"); //This does not take ownership and the variables can be re-used
println!("{}", first_name);


//Common Methods on Strings

let mut music_genres: &str = "    Rock, Metal, Country, Rap    ";
println!("{}", music_genres.trim());//returns a string slice with leading and trailing white space removed
println!("{}", music_genres.trim_start());//returns a string slice with the leading white space removed
println!("{}", music_genres.trim_end()); //returns a string slice with the trailing white space removed

music_genres = music_genres.trim();//trims leading and trailing white space and assigns to variable
println!("{}", music_genres.to_uppercase());
println!("{}", music_genres.to_lowercase());

//replace() takes 2 arguments, what and what it is be replaced with
println!("{}", music_genres.replace("a", "@"));//creates a new heap allocated string where every a is replaced with @

//split allows us to split the string based on a specified delimiter
let genres: Vec<&str> = music_genres.split(", ").collect();//", " is the specified delimiter that we are splitting on. If we want to collect all the pieces into a vector, we define the specific Vec<Type> and use the .collect() method on the split struct
println!("{:?}", genres);//will output ["Rock", "Metal", "Country", "Rap"], split is helpful if you are splitting addresses based on commas, first name and last name separated by a space, etc.
}
