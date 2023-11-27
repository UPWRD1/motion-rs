use std::{vec::*, collections::VecDeque};

pub fn grow_capacity(capacity: usize) -> usize {
    if capacity < 8 {
        8
    } else {
        capacity * 2
    }
}

pub fn grow_array<T: Clone + std::default::Default>(vec: &VecDeque<T>, old_count: usize, new_count: usize) -> VecDeque<T> {
    let mut new_vec = Vec::with_capacity(new_count);
    // Ensure that the length of the new vector is set appropriately
    let binding = std::default::Default::default();
    new_vec.resize(new_count, &binding);
    // Clone the elements from the old vector to the new one
    new_vec[..old_count].clone_from_slice(&vec.iter().collect::<Vec<&T>>()[..old_count]);
    // Manually initialize the additional elements (if any)
    for item in new_vec.iter_mut().take(new_count).skip(old_count) {
        *item = &binding;
    }
    let fvec = VecDeque::from(new_vec).iter().map(|x| -> T {x.to_owned().clone()}).collect();
    fvec
}

pub fn free_array<T>(vec: &mut VecDeque<T>, _old_count: usize) -> &VecDeque<T> {
    // Dropping the vector will deallocate its memory
    vec.clear();
    vec
}
