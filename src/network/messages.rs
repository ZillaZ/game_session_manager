use deku::prelude::*;

#[derive(DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum Reason {
    #[deku(id = "0x1")]
    IdInUse,
    #[deku(id = "0x2")]
    InvalidRequestFormat,
    #[deku(id = "0x3")]
    InvalidIdFormat,
    #[deku(id = "0x4")]
    InvalidPassword,
    #[deku(id = "0x5")]
    IdDoesntExist,
    #[deku(id = "0x6")]
    WrongPassword
}

impl ToString for Reason {
    fn to_string(&self) -> String {
        use Reason::*;
        match self {
            IdInUse => "The given ID is already in use.".into(),
            InvalidRequestFormat => "The request is invalid.".into(),
            InvalidIdFormat => "The given ID is invalid".into(),
            InvalidPassword => "The given password is invalid".into(),
            IdDoesntExist => "There is no session with the given ID".into(),
            WrongPassword => "The given password is incorrect".into()
        }
    }
}

#[derive(DekuRead, DekuWrite)]
pub struct ContainerInfo {
    #[deku(update = "self.addr.len()")]
    count: usize,
    #[deku(count = "count")]
    pub addr: Vec<u8>
}

impl ContainerInfo {
    pub fn new(addr: &str) -> Self {
        let addr = addr.as_bytes().to_vec();
        Self {
            count: addr.len(),
            addr
        }
    }
}

#[derive(DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum ServerResponse {
    #[deku(id = "0x1")]
    Ok(ContainerInfo),
    #[deku(id = "0x2")]
    InvalidRequest(Reason),
}

#[derive(DekuRead, DekuWrite)]
pub struct NewSessionRequest {
    #[deku(update = "self.id.len()")]
    id_count: usize,
    #[deku(count = "id_count")]
    pub id: Vec<u8>,
    #[deku(update = "self.password.len()")]
    count: usize,
    #[deku(count = "count")]
    pub password: Vec<u8>,
    pub player_limit: u8,
}

#[derive(DekuRead, DekuWrite)]
pub struct JoinSessionRequest {
    #[deku(update = "self.id.len()")]
    id_count: usize,
    #[deku(count = "id_count")]
    pub id: Vec<u8>,
    #[deku(update = "self.password.len()")]
    count: usize,
    #[deku(count = "count")]
    pub password: Vec<u8>,
}

#[derive(DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum PlayerRequest {
    #[deku(id = "0x1")]
    NewSession(NewSessionRequest),
    #[deku(id = "0x2")]
    JoinSession(JoinSessionRequest),
}

#[derive(DekuRead, DekuWrite)]
#[deku(type = "u8")]
pub enum JoinResponse {
    #[deku(id = "0x1")]
    Ok,
    #[deku(id = "0x2")]
    Err(Reason),
}
