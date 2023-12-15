#[derive(Debug)]
pub struct ProxmoxAuthentication {
    pub user: String,
    pub realm: String,
    pub token: PveToken,
}

#[derive(Debug)]
pub struct PveToken {
    pub name: String,
    pub value: String,
}
