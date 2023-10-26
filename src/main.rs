mod chapter2;

fn main() {
    let list = [1, 3, 5, 7, 9,11,13,52,78];
    let item = 1;

    let result = binary_search(&list, item);

    match result {
        Some(index) => println!("Элемент {} найден на позиции {}.", item, index),
        None => println!("Элемент {} не найден в списке.", item),
    }
}

fn binary_search(list: &[i32; 9], item: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len()-1;

    while low <= high {
        let mid = (low+high)/2;
        if list[mid]==item {
            return Some(mid);
        } else if list[mid] < item {
            low=mid+1;
        } else {
            high = mid - 1;
        }
    }
    None
}