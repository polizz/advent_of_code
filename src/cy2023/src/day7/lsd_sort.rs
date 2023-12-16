#![allow(unused)]
#![allow(non_snake_case)]

use std::alloc::{alloc, handle_alloc_error, Layout};
use std::collections::HashMap;

fn check_override_mapping(map: &HashMap<char, char>, c: usize) -> usize {
    if let Some(resultant) = map.get(&(c as u8 as char)) {
        // println!(
        //     "Mappped:{} to {}, index will be:{}",
        //     c as u8 as char, *resultant as usize as u8 as char, *resultant as usize
        // );
        return *resultant as usize;
    }

    // println!("No map:{}", c as u8 as char);
    c
}

/*/ Expects normal alpha strings to sort, or can be overriden
with a map from a provided alpha to normalized alpha sort order.
i.e. If you wanted a lexical order of T, U, A vs. the normal A, T, U...
a map of T->A, U-> B, A->C would accomplish this.
*/
use super::Hand;

fn get_object_str_repr(hand: Hand) -> String {
    hand.cards
}

pub(crate) fn sort_hands(mut array: &mut Vec<Hand>, override_sort: Option<&HashMap<char, char>>) {
    let R = 256;
    let mut aux: Vec<Hand> = vec![Hand::default(); R + 1];
    let n_char = array[0].cards.len();
    let n_str = array.len();

    // for each char in the fixed-length strings
    for c in (0..n_char).rev() {
        let mut counts = vec![0usize; R + 1];

        // compute frequency counts for this letter slot for all strings
        for f in 0..n_str {
            // get any overriden index for this the letters in this slot

            let mapped_idx = override_sort.as_ref().map_or_else(
                || array[f].cards.clone().into_bytes()[c] as usize + 1,
                |map| check_override_mapping(&map, array[f].cards.clone().into_bytes()[c] as usize), //+ 1),
            );

            counts[mapped_idx + 1] += 1;
        }

        // transform counts to indices
        for i in 0..R {
            counts[i + 1] += counts[i];
        }

        // move each string into aux in order
        for m in 0..n_str {
            let mapped_idx = override_sort.as_ref().map_or_else(
                || array[m].cards.clone().into_bytes()[c] as usize + 1,
                |map| check_override_mapping(&map, array[m].cards.clone().into_bytes()[c] as usize), //+ 1),
            );

            unsafe {
                let pt = aux.as_mut_ptr().add(counts[mapped_idx]);
                let at = array.as_mut_ptr().add(m);
                std::ptr::swap(at, pt);
            }

            counts[mapped_idx] += 1;
        }

        // move data back to original array
        for c in 0..n_str {
            unsafe {
                let pt = aux.as_mut_ptr().add(c);
                let at = array.as_mut_ptr().add(c);
                std::ptr::swap(at, pt);
            }
        }
    }
}

pub(crate) fn sort(
    mut array: Vec<String>,
    override_sort: Option<HashMap<char, char>>,
) -> Vec<String> {
    // let R = 256;
    let R = 100;
    let mut aux: Vec<Vec<u8>> = vec![vec![0; 1]; R + 1];
    let mut array: Vec<Vec<u8>> = array.into_iter().map(|s| s.into_bytes()).collect();
    let n_char = array[0].len();
    let n_str = array.len();

    // for each char in the fixed-length strings
    for c in (0..n_char).rev() {
        let mut counts = vec![0usize; R + 1];

        // compute frequency counts for this letter slot for all strings
        for f in 0..n_str {
            // get any overriden index for this the letters in this slot
            // dbg!(&array[f]);

            let mapped_idx = override_sort.as_ref().map_or_else(
                || array[f][c] as usize + 1,
                |map| check_override_mapping(&map, array[f][c] as usize), //+ 1),
            );

            // println!(
            //     "Overview: idx: {}, c_index:{}, word:{:?}",
            //     mapped_idx as u8 as char,
            //     c,
            //     String::from_utf8(array[f].clone())
            // );

            // counts[array[f][c] as usize + 1] += 1;
            counts[mapped_idx + 1] += 1;
            // dbg!(&array);
        }

        // println!("Counts: {:?}", &counts);

        // transform counts to indices
        for i in 0..R {
            counts[i + 1] += counts[i];
        }

        // println!("Transform: {:?}", &counts);

        // move each string into aux in order
        for m in 0..n_str {
            // let idx = array[m][c] as usize;
            // println!("Aux ptr add: {}, Arr ptr add: {}", counts[idx], m);
            let mapped_idx = override_sort.as_ref().map_or_else(
                || array[m][c] as usize + 1,
                |map| check_override_mapping(&map, array[m][c] as usize), //+ 1),
            );

            unsafe {
                // println!("pt add: {}, at add: {}", counts[mapped_idx], m);
                let pt = aux.as_mut_ptr().add(counts[mapped_idx]);
                let at = array.as_mut_ptr().add(m);
                std::ptr::swap(at, pt);
            }

            counts[mapped_idx] += 1;
        }

        // dbg!(&aux);
        // println!("array before:{:?}", &array);
        // println!("aux before:{:?}", &aux);

        // move data back to original array
        for c in 0..n_str {
            // println!("Copying {} string over", c);

            unsafe {
                let pt = aux.as_mut_ptr().add(c);
                let at = array.as_mut_ptr().add(c);
                std::ptr::swap(at, pt);
            }
        }

        // println!("array after:{:?}", &array);
        // println!("aux after:{:?}", &aux);
    }

    // SAFETY: this is safe because these are the same
    // bytes that were passed in presumably by safe rust
    unsafe {
        array
            .into_iter()
            .map(|b| String::from_utf8_unchecked(b))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lsd_sort_with_override() {
        let mut override_sort: HashMap<char, char> = HashMap::new();
        override_sort.insert('3' as char, 'Z' as char);
        override_sort.insert('T' as char, 'M' as char);

        let input = vec!["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"];
        let input = input.into_iter().map(|s| s.to_string()).collect();

        let output = sort(input, Some(override_sort));

        let sorted = vec!["KK677", "KTJJT", "T55J5", "QQQJA", "32T3K"];
        let sorted: Vec<String> = sorted.iter().map(|s| s.to_string()).collect();

        assert_eq!(sorted, output);
    }

    #[test]
    fn lsd_sort() {
        let input = vec!["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"];
        let input = input.into_iter().map(|s| s.to_string()).collect();

        let output = sort(input, None);

        let sorted = vec!["32T3K", "KK677", "KTJJT", "QQQJA", "T55J5"];
        let sorted: Vec<String> = sorted.iter().map(|s| s.to_string()).collect();

        assert_eq!(sorted, output);
    }
}
