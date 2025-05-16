/*HashSet Operations - Common mathematical set operations that deal with comparing different HashSets
i.e. finding the values that can be found in both HashSets, finding the values that can be found in one and not the other, etc.
There are a variety of different methods to employ when comparing different HashSets */

use std::collections::HashSet;
fn main() {
    let mut concert_que: HashSet<&str> = HashSet::new();//Boris & Melissa
    let mut movie_que = HashSet::new();//Boris & Phil

    concert_que.insert("Boris");
    concert_que.insert("Melissa");

    movie_que.insert("Boris");
    movie_que.insert("Phil");

    //Union - provides the union, or combination of entry found in both sets, no duplicates. Lists all of the unique entries.
    println!("{:?}", concert_que.union(&movie_que));//call the method on one HashSet, pass in a reference to the other HashSet as an argument
    println!("{:?}", movie_que.union(&concert_que));//output will be the same for both, order may change, but contents remains consistent

    //Difference - Give you the values in the first set, but not found in the second set
    println!("{:?}", concert_que.difference(&movie_que));//["Melissa"] output will change depending on which HashSet is first
    println!("{:?}", movie_que.difference(&concert_que));//["Phil"]

    //Symmetric Difference - provides values that are exclusive to one set, but not found in both. Boris will not be provided in either output
    println!("{:?}", concert_que.symmetric_difference(&movie_que));
    println!("{:?}", movie_que.symmetric_difference(&concert_que));

    //Is Disjoint - Returns a Boolean - True if neither HashSet has any values in Common, False if they do.
    println!("{:?}", concert_que.is_disjoint(&movie_que));
    println!("{:?}", movie_que.is_disjoint(&concert_que));

    //Is Subset - Returns a Boolean. True if the set that the method is invoked upon is a subset of the argument passed in
    println!("{:?}", concert_que.is_subset(&movie_que));
    println!("{:?}", movie_que.is_subset(&concert_que));

    let mut attendees = HashSet::new();
    attendees.insert("Boris");
    println!("{:?}", attendees.is_subset(&concert_que));//This returns true because All values in attendees are obtained in concert_que

    //Is Superset - Inverse of is_subset. Returns true if set method invoked on contains all the values of the passed argument
    println!("{:?}", concert_que.is_superset(&attendees));
    println!("{:?}", attendees.is_superset(&concert_que));


}
