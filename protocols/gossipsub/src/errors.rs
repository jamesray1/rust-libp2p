
use TopicHash;
use libp2p_core::PeerId;
use custom_error::custom_error;
use std::{
    collections::hash_map::HashMap,
    io,
};

custom_error!{pub GError
    Io{source: io::Error} = "Input/output error",
    // Note that when combined with the err arguments passed elsewhere e.g. in mesh and layer, these are repetitive, but avoids ambiguity.
    NotSubscribedToTopic{t_hash: String, peer_id: String, err: String}
        = "The topic with topic hash '{t_hash}' is not in the subscribed \
        topics of the peer with peer id '{peer_id}'.'{err}'",
    NotGraftedToTopic{t_hash: String, peer_id: String, err: String}
        = "The peer with peer id '{peer_id}' is not grafted to the topic. \
        '{err}'",
    TopicNotInMesh{t_hash: String, err: String}
        = "The topic with topic hash '{t_hash}' was not found. '{err}'",
    AlreadyGrafted{t_hash: String, peer_id: String, err: String}
        = "Tried to graft the peer with peer_id '{peer_id}' to the topic \
        with topic hash '{t_hash}' in the mesh, but it is already grafted.",
    InvalidPeerId{from_data: String}
        = "The from field '{from_data}' of an instance of rpc_proto::Message \
        could not be converted to a valid peer ID.",
    // NotConnectedToPeer{peer: String, err: String}
    //     = "The remote peer {peer} was not found in the \
    //     `connected_peers.gossipsub` of the local peer.",
    // NotEnoughPeers{err: String}
    //     = "The local peer is not connected to enough peers.",
}

pub type Result<T> = std::result::Result<T, GError>;

pub struct GraftErrors {
    // Topics that remote peers are not subscribed to (they need to be
    // as a prerequisite to grafting them).
    pub(crate) topics_not_subscribed: Option<HashMap<PeerId, TopicHash>>,
    // Topics that are not in the local peer's mesh view.
    pub(crate) topics_not_in_mesh: Option<Vec<TopicHash>>,
    // Remote peers that are not connected to the local peer.
    pub(crate) r_peers_not_connected: Option<Vec<PeerId>>,
    // Topics that the local peer is already grafted to.
    pub(crate) topics_already_grafted: Option<Vec<TopicHash>>,
    // Whether any of the above are a Some(value).
    pub(crate) has_errors: bool,
}

// pub(crate) struct GraftErrorsForPeer {

// }

impl GraftErrors {
    pub fn new() -> Self {
        GraftErrors {
            topics_not_subscribed: None,
            topics_not_in_mesh: None,
            r_peers_not_connected: None,
            topics_already_grafted: None,
            has_errors: false,
        }
    }
    pub fn is_empty(&self) -> bool {
        if self.has_errors == true {
            return false;
        } else {
            return true;
        }
    }
    // Not used
    // pub fn new_with_not_connected(r_peers_not_connected:
    //     Vec<PeerId>) -> Self {
    //     GraftErrors {
    //         topics_not_subscribed: None,
    //         topics_not_in_mesh: None,
    //         r_peers_not_connected: Some(r_peers_not_connected),
    //         topics_already_grafted: None,
    //     }
    // }
    // pub fn new_with_not_subscribed(
    //     topics_not_subscribed: HashMap<PeerId, TopicHash>) -> Self
    // {
    //     GraftErrors {
    //         topics_not_subscribed: Some(topics_not_subscribed),
    //         topics_not_in_mesh: None,
    //         r_peers_not_connected: None,
    //         topics_already_grafted: None,
    //     }
    // }
    // pub fn new_with_not_in_mesh(
    //     topics_not_in_mesh: Vec<TopicHash>) -> Self
    // {
    //     GraftErrors {
    //         topics_not_subscribed: None,
    //         topics_not_in_mesh: Some(topics_not_in_mesh),
    //         r_peers_not_connected: None,
    //         topics_already_grafted: None,
    //     }
    // }
    // pub fn new_with_not_in_mesh_and_not_subscribed(
    //     topics_not_subscribed: HashMap<PeerId, TopicHash>,
    //     topics_not_in_mesh: Vec<TopicHash>,
    // ) -> Self {
    //     GraftErrors {
    //         topics_not_subscribed: Some(topics_not_subscribed),
    //         topics_not_in_mesh: Some(topics_not_in_mesh),
    //         r_peers_not_connected: None,
    //         topics_already_grafted: None,
    //     }
    // }
    // pub fn add_topics_not_subscribed(&mut self,
    //     topics_not_subscribed: HashMap<PeerId, TopicHash>) {
    //     self.topics_not_subscribed = Some(topics_not_subscribed);
    // }
    // pub fn add_topics_not_in_mesh(&mut self,
    //     topics_not_in_mesh: Vec<TopicHash>) {
    //     self.topics_not_in_mesh = Some(topics_not_in_mesh);
    // }
    // pub fn add_topics_not_in_mesh_and_not_subscribed(&mut self,
    //     topics_not_in_mesh: Vec<TopicHash>,
    //     topics_not_subscribed: HashMap<PeerId, TopicHash>) {
    //     self.topics_not_in_mesh = Some(topics_not_in_mesh);
    //     self.topics_not_subscribed = Some(topics_not_subscribed);
    // }
}