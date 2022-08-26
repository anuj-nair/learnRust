fn median(mut a: Vec<f32>) -> Option<f32> {

    if a.is_empty(){
        return None
    }
    
    a.sort_by(|x: &f32, y: &f32| x.partial_cmp(y).unwrap());

    let n_elements: usize = a.len();
    let middle: usize = n_elements/2;
    
    let med: f32 = if n_elements%2 == 0{
        //even
        (a[middle-1]+a[middle])/2.0
    }else{
        //odd
        a[middle]
    };

    Some(med)
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);
    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
