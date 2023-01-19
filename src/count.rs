pub fn count_entities(query: Query<Entity>) {
    println!("entities: {}", query.iter().count());
}

pub fn count_asteroids(query: Query<&asteroid::Asteroid, Without<Part>>) {
    println!("asteroids: {}", query.iter().count());
}

pub fn count_stars(query: Query<&map::star::Star>) {
    println!("stars: {}", query.iter().count());
}

pub fn count_intercepters(query: Query<&intercepter::Intercepter, Without<Part>>) {
    println!("intercepters: {}", query.iter().count());
}

pub fn count_wreckages(query: Query<&wreckage::Wreckage>) {
    println!("wreckages: {}", query.iter().count());
}

pub fn count_debris(query: Query<&wreckage::WreckageDebris>) {
    println!("debris: {}", query.iter().count());
}
