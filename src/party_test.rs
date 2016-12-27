use std::time::Duration;
use config::Config;
use party::Party;

#[test]
fn single_node() {
    let config = Config {
        party_name: "test_party".to_owned(),
        node_name: "node_1".to_owned(),
        zk_nodes: "localhost:2181".to_owned(),
        timeout: Duration::from_secs(15),
    };

    let mut party = Party::from_config(&config).unwrap();
    party.join().unwrap();
    
    let mut hosts = party.new_hosts().unwrap();
    assert_eq!(hosts, vec!["node_1".to_owned()]);

    hosts = party.hosts().unwrap();
    assert_eq!(hosts, vec!["node_1".to_owned()]);
}
