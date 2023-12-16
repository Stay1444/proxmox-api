# ProxmoxVE API Wrapper for Rust

Check out the examples folder to see how to use it. This library supports login via Proxmox Tokens.

## Supported Proxmox versions

In principle, any starting from Proxmox 6.1. But it was tested on Proxmox 8.0

## Quick Example

This example queries the available nodes in the Proxmox cluster and iterates over them

```rs
use proxmox_api::{
  auth::{ProxmoxAuthentication, PveToken},
  ProxmoxClient
};

#[tokio::main]
async fn main() {
  let auth = ProxmoxAuthentication {
    user: "root".into(),
    realm: "pam".into(),

    token: PveToken {
      name: "example-token",
      value: "YOUR-TOKEN-HERE"
    },
  };

  let client = ProxmoxClient::new("https://0.0.0.0:8006".parse().unwrap(), auth);

  for node in client.nodes().await.unwrap() {
    println!("----");
    println!("Name: {}", node.id);
    println!("Status: {}", node.status);
  }
}
```

