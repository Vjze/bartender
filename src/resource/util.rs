use tiberius::{error::Error, AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};
pub async fn client(sql: String) -> Result<Client<Compat<TcpStream>>, Error> {
    let mut config = Config::new();
    config.host(sql);
    config.port(1433);
    config.database("TESTDATA");
    // config.authentication(AuthMethod::sql_server("cytest", "cytest"));
    config.authentication(AuthMethod::sql_server("hztest", "hztest"));
    config.trust_cert();
    // let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
    let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
    tcp.set_nodelay(true).unwrap();
    let client = tiberius::Client::connect(config, tcp.compat_write()).await;
    client
}
