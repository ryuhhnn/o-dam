use rocket_contrib::databases::rusted_cypher::GraphClient;

#[database("neo_store")]
pub struct NeoStore(GraphClient);
