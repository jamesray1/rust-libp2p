use TopicHash;
use errors::{GError, Result as GResult};

use libp2p_core::PeerId;

use std::{
    collections::hash_map::HashMap
    };

/// A soft overlay network for topics of interest, which meshes as a map
/// of topics to lists of peers. It is a randomized topic mesh as a map of a
/// topic to a list of peers. The local peer maintains a topic view of its
/// direct peers only, a subset of the total peers that subscribe to a topic,
/// in order to limit bandwidth and increase decentralization, security and
/// sustainability. Extracts from the
/// [spec](https://github.com/libp2p/specs/tree/master/pubsub/gossipsub)
/// are as follows, although FMI read the full spec:
/// > The overlay is maintained by exchanging subscription control messages
/// > whenever there is a change in the topic list. The subscription
/// > messages are not propagated further, so each peer maintains a topic
/// > view of its direct peers only. Whenever a peer disconnects, it is
/// > removed from the overlay…
/// > We can form an overlay mesh where each peer forwards to a subset of
/// > its peers on a stable basis… Each peer maintains its own view of the
/// > mesh for each topic, which is a list of bidirectional links to other
/// > peers. That is, in steady state, whenever a peer A is in the mesh of
/// > peer B, then peer B is also in the mesh of peer A.
///
/// > **Note**: as discussed in the spec, ambient peer discovery is pushed
/// > outside the scope of the protocol.
#[derive(Debug)]
pub struct Mesh { m: HashMap<TopicHash, Vec<PeerId>> }

impl Mesh {
    /// Creates a new `Mesh`.
    pub(crate) fn new() -> Self {
        Mesh {
            m: HashMap::new(),
        }
    }

    /// Inserts a topic via it's `TopicHash` and grafted peers to the mesh.
    pub(crate) fn insert(&mut self, k: TopicHash, v: Vec<PeerId>)
        -> Option<Vec<PeerId>> {
        self.m.insert(k, v)
    }

    /// Gets all the peers that are grafted to a topic in the mesh, or returns
    /// an error if the topic is not in the mesh.
    pub(crate) fn get_peers_from_topic(&self, th: &TopicHash)
        -> GResult<Vec<PeerId>>
    {
        let th_str = th.clone().into_string();
        match self.m.get(th) {
            Some(peers) => {return Ok(peers.to_vec());},
            None => {return Err(GError::TopicNotInMesh{t_hash: th_str,
                err: "Tried to get peers from the topic with topic hash /
                '{&th_str}' but this topic is not found in the mesh."
                .to_string()});}
        }
    }

    /// Gets a peer that is grafted to a topic in the mesh, or returns a
    /// `GError` if the peer or topic is not in the mesh.
    pub(crate) fn get_peer_from_topic(&self, th: TopicHash, p: PeerId)
        -> GResult<PeerId> {
        let get_result = self.get_peers_from_topic(&th).map(|peers| {
            match peers.into_iter().find(|peer| peer == &p) {
                Some(peer) => return Ok(peer),
                None => {
                    let th_str = th.clone().into_string();
                    Err(GError::NotGraftedToTopic{t_hash: th_str,
                        peer_id: p.clone().to_base58(),
                        err: "Tried to get peer '{p}' but it was not found \
                        in the peers that are grafted to the topic with topic hash \
                        '{&th_str}'.".to_string()})
                }
            }
        });
        match get_result {
            Ok(result) => result,
            Err(err) => Err(err),
        }
    }

    // Graft
    pub(crate) fn add_peer(&mut self, th: TopicHash, p: PeerId) {
        self.m.entry(th).and_modify(|ps| ps.push(p));
    }

    // pub fn get_mut(&mut self, ) {}

    pub(crate) fn remove(&mut self, th: &TopicHash) -> GResult<Vec<PeerId>>
    {
        if let Some(peers) = self.m.remove(th) {
            Ok(peers)
        } else {
            let th_str = th.clone().into_string();
            Err(GError::TopicNotInMesh{t_hash: th_str,
            err: "Tried to remove the topic with topic hash '{&th_str}' from \
            the mesh.".to_string()})
        }
    }

    // Prune with handling
    pub(crate) fn remove_peer_from_topic(&mut self, th: &TopicHash,
        p: PeerId) -> GResult<()>
    {
        let peer_str = p.to_base58();
        let th_str = th.clone().into_string();
        let no_t = GError::TopicNotInMesh{t_hash: th_str.clone(),
                err: "Tried to remove the topic with topic hash '{&th_str}' \
                from the mesh.".to_string()};
        match self.remove(th) {
            Ok(mut peers) => {
                // TODO: use remove_item when stable:
                // https://github.com/rust-lang/rust/issues/40062
                for (pos, peer) in peers.clone().iter().enumerate() {
                    if peer.clone() == p {
                        // prune
                        peers.remove(pos);
                        // The same peer ID cannot exist more than
                        // once in the vector, since we check if the peer
                        // already exists before adding it in
                        // the graft methods.
                        return Ok(());
                    }
                }
                return Err(GError::NotGraftedToTopic{
                    t_hash: th_str.clone(), peer_id: peer_str.to_string(), err:
                    "Tried to remove the peer '{peer_str}' from the topic \
                    with topic hash '{&th_str}'.".to_string()});
            },
            Err(no_t) => {
                return Err(GError::TopicNotInMesh{t_hash: th_str.clone(),
                err: "Tried to remove the peer with id '{&peer_str}' from the \
                topic with topic hash '{&th_str}' from the mesh, but the \
                topic was not found.".to_string()})
            },
        }
    }
}
