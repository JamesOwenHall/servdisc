use std::thread;
use std::time::Duration;
use config::Config;
use party::Party;

#[test]
fn single_node() {
    let mut party = Party::from_config(&test_config()).unwrap();
    party.join().unwrap();
    
    let hosts = party.new_hosts().unwrap();
    assert_eq!(hosts, vec!["node_1"]);
}

#[test]
fn three_nodes() {
    // Create first party.
    let mut party1 = Party::from_config(&test_config()).unwrap();
    party1.join().unwrap();

    let mut exp = vec!["node_1"];
    let mut hosts = party1.new_hosts().unwrap();
    assert_eq!(hosts, exp);

    // Create second party.
    let mut config2 = test_config();
    config2.node_name = "node_2".to_owned();
    let mut party2 = Party::from_config(&config2).unwrap();
    party2.join().unwrap();

    exp = vec!["node_1", "node_2"];
    hosts = party1.new_hosts().unwrap();
    assert_eq!(hosts, exp);
    hosts = party2.new_hosts().unwrap();
    assert_eq!(hosts, exp);

    // Create third party.
    let mut config3 = test_config();
    config3.node_name = "node_3".to_owned();
    let mut party3 = Party::from_config(&config3).unwrap();
    party3.join().unwrap();   

    exp = vec!["node_1", "node_2", "node_3"];
    hosts = party1.new_hosts().unwrap();
    assert_eq!(hosts, exp);
    hosts = party2.new_hosts().unwrap();
    assert_eq!(hosts, exp);
    hosts = party3.new_hosts().unwrap();
    assert_eq!(hosts, exp);

    // Drop node_2.
    drop(party2);
    exp = vec!["node_1", "node_3"];
    hosts = party1.new_hosts().unwrap();
    assert_eq!(hosts, exp);
    hosts = party3.new_hosts().unwrap();
    assert_eq!(hosts, exp);
}

#[test]
fn three_nodes_multithreaded() {
    // Create first and second parties.
    let mut party1 = Party::from_config(&test_config()).unwrap();
    party1.join().unwrap();

    let mut config2 = test_config();
    config2.node_name = "node_2".to_owned();
    let mut party2 = Party::from_config(&config2).unwrap();
    party2.join().unwrap();

    let mut exp = vec!["node_1", "node_2"];
    let mut hosts = party1.new_hosts().unwrap();
    assert_eq!(hosts, exp);
    hosts = party2.new_hosts().unwrap();
    assert_eq!(hosts, exp);

    // Move first and second parties into new thread.
    let handle = thread::spawn(move || {
        let exp = vec!["node_1", "node_2", "node_3"];
        let mut hosts = party1.new_hosts().unwrap();
        assert_eq!(hosts, exp);
        hosts = party2.new_hosts().unwrap();
        assert_eq!(hosts, exp);
    });

    // Create third party.
    let mut config3 = test_config();
    config3.node_name = "node_3".to_owned();
    let mut party3 = Party::from_config(&config3).unwrap();
    party3.join().unwrap();

    exp = vec!["node_1", "node_2", "node_3"];
    hosts = party3.new_hosts().unwrap();
    assert_eq!(hosts, exp);

    handle.join().unwrap();
}

fn test_config() -> Config {
    Config {
        party_name: "test_party".to_owned(),
        node_name: "node_1".to_owned(),
        zk_nodes: "localhost:2181".to_owned(),
        timeout: Duration::from_secs(15),
    }
}
