// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.



/// Clippy suggère de supprimer l'appel à `unwrap()` car il va toujours paniker
/// puisque `my_option` est `None`
/// if my_option.is_none() {
///     my_option.unwrap();
/// Clippy a détecté qu'il manquait une virgule dans la déclaration du tableau
/// Clippy suggère de supprimer cette ligne car elle n'a pas d'effet
/// let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
/// Ou on peut aussi utiliser directement `vec![]` avec le type explicité
/// Clippy suggère d'utiliser `std::mem::swap` pour échanger les valeurs
/// au lieu de les affecter manuellement

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    println!("This Vec is empty, see? {:?}", Vec::<i32>::new());
    println!("This Vec is empty, see? {:?}", vec![] as Vec<i32>);

    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
