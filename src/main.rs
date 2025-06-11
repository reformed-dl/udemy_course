/* Traits - A distinguishing quality, characteristic, function or ability. 

Real World Examples
Flight is a trait, the quality of being able to fly. Birds, planes, bugs, etc 
Storage Trait - garage, boxes, USB drive. All different types of things, but express the same Trait of storage
Illumination Trait - candle, lamp, sun

Just as in the real world, many different Types in Rust can implement the same Trait
A Trait is a contract that describes the functionality that a Type should have.
We use the word "implement" to describe when a type honors a Trait's requirements. (The sun implements the illumination trait)

A Trait definition declares the method(s) that a Type implementing that trait MUST have
    Method Name, Parameters with Types, Return Value Type

Common Examples we have seen already: 
Display and Debug require a Type define methods for formatting itself in a string representation
Clone requires a Type to define a clone method for creating a duplicate of itself

Once we have defined a Trait, we can implement it on structs and enums. The Type then promises to honor the Trait's contract.
Multiple Types can implement the same Trait.
Types can implement multiple Traits.
*/

fn main() {
  
}

 