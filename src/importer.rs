// importer.rs
use crate::models::{
    config::Config,
    csv::{Edge, Node},
};
use gremlin_client::{aio::GremlinClient, process::traversal::traversal, GremlinError, Vertex};

pub async fn import_nodes(
    config: &Config,
    nodes: &[Node],
) -> Result<(), Box<dyn std::error::Error>> {
    let client = GremlinClient::connect(config.hosts[0].as_str()).await?;
    let g = traversal().with_remote_async(client);

    for node in nodes {
        let mut traversal = g.add_v(node.label.as_str());

        for (key, value) in &node.properties {
            traversal = traversal.property(key.as_str(), value.as_str());
        }

        let _vertex: Vertex = traversal.to_list().await?.pop().unwrap();
    }
    Ok(())
}

pub async fn import_edges(config: &Config, edges: &[Edge]) -> Result<(), Box<GremlinError>> {
    let client = GremlinClient::connect(config.hosts[0].as_str()).await?;
    let g = traversal().with_remote_async(client);

    for edge in edges {
        let from_vertex: Vertex = g.v(edge.from.as_str().to_owned()).next().await?.unwrap();

        let to_vertex: Vertex = g.v(edge.to.as_str().to_owned()).next().await?.unwrap();
        let _e = g
            .add_e(edge.relationship.as_str())
            .from(&from_vertex)
            .to(&to_vertex)
            .iter()
            .await?;
    }
    Ok(())
}
