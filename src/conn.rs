use cdrs::{
    authenticators::NoneAuthenticator,
    cluster::session::{new as new_session, Session},
    cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool},
    load_balancing::RoundRobinSync,
};

use async_trait::async_trait;

use crate::{ConnectionError, Connection};

type CQLSession = Session<RoundRobinSync<TcpConnectionPool<NoneAuthenticator>>>;

// TODO: Error handling
#[async_trait]
impl Connection<CQLSession> for CQLSession {
    async fn establish_connection(url: &str) -> Result<CQLSession, ConnectionError> {
        let node = NodeTcpConfigBuilder::new(url, NoneAuthenticator {}).build();
        let cluster = ClusterTcpConfig(vec![node]);
        let balance = RoundRobinSync::new();
        let conn: CQLSession = new_session(&cluster, balance).expect("session should be created");

        Ok(conn)
    }

    async fn destroy_connection(_connection: CQLSession) -> Result<(), ConnectionError> {
        Ok(())
    }
}
