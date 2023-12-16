use proxmox_api::{
    auth::{ProxmoxAuthentication, PveToken},
    ProxmoxClient,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt().pretty().init();

    let auth = ProxmoxAuthentication {
        user: "root".into(),
        realm: "pam".into(),
        token: PveToken {
            name: "api-test".into(),
            value: "c0c192e7-a24d-4947-b4fa-7db9ba1b8a6b".into(),
        },
    };

    let client = ProxmoxClient::new("https://172.10.0.2:8006".parse().unwrap(), auth);

    let node = client.node("avc01");

    dbg!(node.version().await.unwrap());
    dbg!(node.vzdump.defaults(None).await);
}
