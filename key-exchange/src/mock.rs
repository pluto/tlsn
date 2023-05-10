//! This module provides mock types for key exchange leader and follower and a function to create
//! such a pair

use crate::{config::KeyExchangeConfigBuilder, KeyExchangeCore, KeyExchangeMessage, Role};

use mpc_garble::{Decode, Execute, Memory};
use point_addition::mock::{
    create_mock_point_converter_pair, MockPointConversionReceiver, MockPointConversionSender,
};
use utils_aio::duplex::DuplexChannel;

/// A mock key exchange instance
pub type MockKeyExchange<E> =
    KeyExchangeCore<MockPointConversionSender, MockPointConversionReceiver, E>;

/// Create a mock pair of key exchange leader and follower
pub fn create_mock_key_exchange_pair<E: Memory + Execute + Decode + Send>(
    leader_executor: E,
    follower_executor: E,
) -> (MockKeyExchange<E>, MockKeyExchange<E>) {
    let (leader_pa_sender, follower_pa_recvr) = create_mock_point_converter_pair();
    let (follower_pa_sender, leader_pa_recvr) = create_mock_point_converter_pair();

    let (leader_channel, follower_channel) = DuplexChannel::<KeyExchangeMessage>::new();

    let key_exchange_config_leader = KeyExchangeConfigBuilder::default()
        .id(String::from(""))
        .role(Role::Leader)
        .build()
        .unwrap();

    let key_exchange_config_follower = KeyExchangeConfigBuilder::default()
        .id(String::from(""))
        .role(Role::Follower)
        .build()
        .unwrap();

    let leader = KeyExchangeCore::new(
        Box::new(leader_channel),
        leader_pa_sender,
        leader_pa_recvr,
        leader_executor,
        key_exchange_config_leader,
    );

    let follower = KeyExchangeCore::new(
        Box::new(follower_channel),
        follower_pa_sender,
        follower_pa_recvr,
        follower_executor,
        key_exchange_config_follower,
    );

    (leader, follower)
}