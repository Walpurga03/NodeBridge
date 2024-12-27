pub mod dashboard;
pub mod block_details;
pub mod mempool;
pub mod network;
pub mod peer_list;
pub mod mining;
pub mod security;
pub mod explorer;

pub use dashboard::render as render_node_info;
pub use block_details::render as render_block_details;
pub use mempool::render as render_mempool;
pub use network::render as render_network;
pub use peer_list::render as render_peer_list;
pub use mining::render as render_mining;
pub use security::render as render_security;
pub use explorer::render as render_explorer; 