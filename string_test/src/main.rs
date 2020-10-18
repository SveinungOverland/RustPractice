use std::mem;



fn main() {

    let story_string = String::from("Hello world!");

    let mut story = mem::ManuallyDrop::new(story_string);

    let ptr: *mut u8 = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(12, len);

    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(String::from("Hello world!"), s);
}
