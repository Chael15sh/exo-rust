// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.



#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second= numbers.1;
    //Un tuple est une collection de valeurs, similaire à un tableau, mais avec une taille fixe.
    //Les éléments d'un tuple sont indexés à partir de 0.
    //Donc pour accéder au deuxième élément d'un tuple, on utilise l'index 1, car le comptage commence à 0
    
    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
