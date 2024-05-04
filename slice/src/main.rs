fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    /* Rather than a reference to the entire String, hello is a reference to a
    portion of the String, specified in the extra [0..5] bit. We create slices
    using a range within brackets by specifying [starting_index..ending_index],
    where starting_index is the first position in the slice and ending_index is
    one more than the last position in the slice. Internally, the slice data
    structure stores the starting position and the length of the slice, which
    corresponds to ending_index minus starting_index. So, in the case of let
    world = &s[6..11];, world would be a slice that contains a pointer to the
    byte at index 6 of s with a length value of 5. */

    println!("{:?}, {:?}!", hello, world);
}
