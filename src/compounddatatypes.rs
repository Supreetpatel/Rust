//Compound datatypes: (arrays,tuples,slices,strings(slice string))

pub fn compounddatatypes() {
//Arrays:-
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", numbers);
    let fruits: [&str;3] = ["apple","banana","orange"];
    println!("Fruit array: {:?}", fruits);
    println!("Fruit array in 1st element: {}", fruits[0]);
    println!("Fruit array in 2nd element: {}", fruits[1]);
    println!("Fruit array in 3rd element: {}", fruits[2]);
//Tuples:-
    let human: (&str, bool, i32) = ("Alice",true,30);
    println!("Human data:{:?}",human);
//to convert into string,we have put ".to_string()"
    let human: (String, bool, i32) = ("Alice".to_string(),true,30);
    println!("Human data:{:?}",human);
//Mix tuples:-
    let my_mix_tuple: (&str, i32, bool, [i32; 5]) = ("Kratos",23,true,[1,2,3,4,5]);
    println!("My mix tupples:{:?}",my_mix_tuple);
//Slices:- It is a two-word object; the first word is a pointer to the data, the second word is the length of the slice.
    let slices: &[i32; 5] = &[1,2,3,4,5];
    println!("Numbers slice:{:?}",slices);
    let animals_slices: &[&str; 3] = &["lion","tiger","elephant"];
    println!("Animals slice:{:?}",animals_slices);
    let books_slices: &[String; 3] = &["Harry Potter".to_string(),"Rich Dad Poor Dad".to_string(),"I don't love you naymore".to_string()];
    println!("Books slice:{:?}",books_slices);
//Strings vs String Slices(&str):-
}

//for the debuggable fromat we have to put "{:?}" to run the arrays.