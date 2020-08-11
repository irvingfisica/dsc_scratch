#[derive(Debug)]
struct  Datero {
    id: usize,
    name: String,
    friends: Vec<usize>,
}

impl Datero {
    fn number_of_friends(&self) -> i32 {
        self.friends.len() as i32
    }
}


fn main() {

    let usuarios = [
        "Hero",
        "Dunn",
        "Sue",
        "Chi",
        "Thor",
        "Clive",
        "Hicks",
        "Devin",
        "Kate",
        "Klein",
    ];

    let friendships:[(usize,usize);12] = [(0,1),(0,2),(1,2),(1,3),(2,3),(3,4),(4,5),(5,6),(5,7),(6,8),(7,8),(8,9)];

    let mut users: Vec<Datero> = Vec::new(); 

    for (i,usuario) in usuarios.iter().enumerate() {
        users.push(Datero {
            id: i,
            name: usuario.to_string(),
            friends: vec![],
        });
    }

    for (i,j) in friendships.iter() {
        users[*i].friends.push(*j);
        users[*j].friends.push(*i);
    }

    let mut suma = 0;
    for usuario in users.iter(){
        suma += usuario.number_of_friends();
    }

    println!("El número total de conecciones es {}", suma);
    println!("El promedio de conecciones es {}", suma as f64 / users.len() as f64);

    let mut num_friends_by_id = Vec::new();

    for usuario in users.iter(){
        num_friends_by_id.push((usuario.id,usuario.number_of_friends()))
    }
    
    println!("{:?}",num_friends_by_id);

    num_friends_by_id.sort_by(|a,b| b.1.cmp(&a.1));

    println!("{:?}",num_friends_by_id);
}
