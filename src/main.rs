use std::collections::HashMap;

//Estructura básica de nodo en la red
#[derive(Debug)]
struct  Nodo {
    id: usize,
    name: String,
    neighbours: Vec<usize>,
}

//Funciones agregadas al nodo
impl Nodo {
    fn number_of_links(&self) -> i32 {
        self.neighbours.len() as i32
    }

    fn neighbours_2lev(&self,red:&HashMap<usize,Nodo>) -> HashMap<usize,usize> {
        let mut neighs_of_neigh:HashMap<usize,usize> = HashMap::new();
        for neighbour in &self.neighbours{
            if let Some(x) = red.get(neighbour) {
                for neigh in &x.neighbours{
                    if neigh != &self.id && !self.neighbours.contains(neigh) {
                        *neighs_of_neigh.entry(*neigh).or_insert(0) += 1;
                    }
                }
            }
        };
        neighs_of_neigh
    }
}

fn main() {
    //Nodos iniciales
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
        "Hellen",
    ];

    //Enlaces iniciales
    let relationships:[(usize,usize);12] = [(0,1),(0,2),(1,2),(1,3),(2,3),(3,4),(4,5),(5,6),(5,7),(6,8),(7,8),(8,9)];

    //Creación de la red
    let mut red: HashMap<usize,Nodo> = HashMap::new();

    for (i,usuario) in usuarios.iter().enumerate() {

        red.entry(i)
            .or_insert(Nodo {
                id: i,
                name: usuario.to_string(),
                neighbours: vec![],
            });

    }

    for (i,j) in relationships.iter() {

        if let Some(x) = red.get_mut(i) {
            x.neighbours.push(*j);
        }

        if let Some(x) = red.get_mut(j) {
            x.neighbours.push(*i);
        }

    }

    println!("{:?}",red);

    //Número total de enlaces y número de enlaces promedio
    let mut suma = 0;
    for (_,usuario) in red.iter(){
        suma += usuario.number_of_links();
    }

    println!("El número total de enlaces es {}", suma);
    println!("El promedio de enlaces es {}", suma as f64 / red.len() as f64);

    //Número de enlaces por ID
    let mut num_links_by_id = Vec::new();

    for (llave,usuario) in red.iter(){
        num_links_by_id.push((llave,usuario.number_of_links()))
    }
    
    println!("{:?}",num_links_by_id);

    //Ordenados de mayor a menor número de enlaces
    num_links_by_id.sort_by(|a,b| b.1.cmp(&a.1));

    println!("{:?}",num_links_by_id);

    if let Some(x) = red.get(&3) {
        println!("{:?}",x.neighbours_2lev(&red));
    }
}
