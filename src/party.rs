use std::sync::mpsc;
use config::Config;
use zookeeper::{CreateMode, Watcher, WatchedEvent, ZkError, ZkResult, ZooKeeper};
use zookeeper::acls;

pub struct Party {
    name: String,
    node_name: String,
    zk: ZooKeeper,
    nodes: Vec<String>,
}

impl Party {
    pub fn from_config(config: &Config) -> ZkResult<Self> {
        let zk = ZooKeeper::connect(
            &config.zk_nodes,
            config.timeout,
            DiscardWatcher,
        )?;

        let party = Party {
            name: config.party_name.clone(),
            node_name: config.node_name.clone(),
            zk: zk,
            nodes: Vec::new(),
        };

        // Create /parties/{party_name}/nodes
        party.create_if_absent(&Self::parties_path(), CreateMode::Persistent)?;
        party.create_if_absent(&party.party_path(), CreateMode::Persistent)?;
        party.create_if_absent(&party.nodes_path(), CreateMode::Persistent)?;

        Ok(party)
    }

    pub fn join(&self) -> ZkResult<String> {
        self.create(&self.node_path(), CreateMode::Ephemeral)
    }

    pub fn hosts(&mut self) -> ZkResult<Vec<String>> {
        let children = self.zk.get_children(&self.nodes_path(), false)?;
        if children != self.nodes {
            self.nodes = children;
        }

        Ok(self.nodes.clone())
    }

    pub fn new_hosts(&mut self) -> ZkResult<Vec<String>> {
        loop {
            let (sender, receiver) = mpsc::channel();
            let children = self.zk.get_children_w(&self.nodes_path(), move |_| {
                sender.send(()).ok();
            })?;

            if children != self.nodes {
                self.nodes = children;
                return Ok(self.nodes.clone());
            }

            receiver.recv().unwrap();
        }
    }

    fn create(&self, path: &str, mode: CreateMode) -> ZkResult<String> {
        let acl = acls::OPEN_ACL_UNSAFE.clone();
        self.zk.create(path, vec![], acl, mode)
    }

    fn create_if_absent(&self, path: &str, mode: CreateMode) -> ZkResult<String> {
        match self.create(path, mode) {
            Ok(s) => Ok(s),
            Err(ZkError::NodeExists) => Ok(String::from(path)),
            Err(e) => Err(e),
        }
    }

    fn append_path(mut dir: String, child: &str) -> String {
        dir.push('/');
        dir.push_str(child);
        dir
    }

    // /parties
    fn parties_path() -> String {
        String::from("/parties")
    }

    // /parties/{party_name}
    fn party_path(&self) -> String {
        Self::append_path(Self::parties_path(), &self.name)
    }

    // /parties/{party_name}/nodes
    fn nodes_path(&self) -> String {
        Self::append_path(self.party_path(), "nodes")
    }

    // /parties/{party_name}/nodes/{node_name}
    fn node_path(&self) -> String {
        Self::append_path(self.nodes_path(), &self.node_name)
    }
}

#[derive(Debug)]
struct DiscardWatcher;
impl Watcher for DiscardWatcher {
    fn handle(&self, _: WatchedEvent) {}
}
