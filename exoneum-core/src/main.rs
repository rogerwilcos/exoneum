extern crate exonum;
extern crate exonum_configuration;
extern crate exonum_time;
extern crate exoneum_core;

use exonum::helpers;
use exonum::helpers::fabric::NodeBuilder;
use exonum_time::TimeServiceFactory;
use exonum_configuration::ServiceFactory as ConfigurationServiceFactory;
use exoneum_core::ExoneumCoreServiceFactory;

fn main() {

    // need to autentification user by pass ?

    exonum::crypto::init();
    helpers::init_logger().unwrap();
    let node = NodeBuilder::new()
        .with_service(Box::new(ConfigurationServiceFactory))
        .with_service(Box::new(TimeServiceFactory))
        .with_service(Box::new(ExoneumCoreServiceFactory));
    node.run();

    // need to check local db and configurations. 
    // If it exists - do nothing
    // If it doesn't exist - create user

    // parse args and configurate ExonumCoreService
    // --name {username}
    // 'pub_key', 'balance' and 'user_address' generated automatically
}
