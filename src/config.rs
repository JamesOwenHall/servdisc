use std::time::Duration;

#[derive(Clone,Debug,PartialEq)]
pub struct Config {
    pub party_name: String,
    pub node_name: String,
    pub zk_nodes: String,
    pub timeout: Duration,
}
