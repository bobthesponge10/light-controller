pub fn mut_vec_to_vec_mut<T>(v: &mut Vec<T>) -> Vec<&mut T>{
    let s = v.len();

    let mut mut_slices_iter = v.iter_mut();
    let mut mut_slices = Vec::with_capacity(s);
    for _ in 1..s{
        mut_slices.push(
            match mut_slices_iter.next(){Some(x) => x, _ => break}
        );
    }
    return mut_slices;
}
pub fn ref_vec_to_vec_ref<T>(v: &Vec<T>) -> Vec<&T>{
    return v
        .into_iter()
        .map(|x| x)
        .collect();
}