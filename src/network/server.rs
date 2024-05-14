use std::{net::{TcpStream, TcpListener}, io::{Read, Write}};
use deku::{DekuContainerRead, DekuContainerWrite};
use crate::database::manager::DatabaseManager;
use super::messages::{PlayerRequest, JoinSessionRequest, NewSessionRequest, ServerResponse};

pub struct Server {
    listener: TcpListener,
    database: DatabaseManager
}

impl Server {
    pub fn new(addr: &str, db_addr: &str) -> Self {
        Self {
            listener : std::net::TcpListener::bind(addr).unwrap(),
            database: DatabaseManager::new(db_addr)
        }
    }
    pub fn accept_incoming(&self) {
        while let Ok((mut stream, _)) = self.listener.accept() {
            let mut buffer = [0; 1024];
            let _count = stream.read(&mut buffer);
            if let Ok((_, player_request)) = PlayerRequest::from_bytes((&buffer, 0)){
                match player_request {
                    PlayerRequest::NewSession(request) => {
                        self.try_create_game(request, stream);
                    },
                    PlayerRequest::JoinSession(request) => {
                        self.try_join_game(request, stream);
                    }
                }
            }
        }
    }
    fn try_create_game(&self, request: NewSessionRequest, mut stream: TcpStream) {
        let id = String::from_utf8(request.id).unwrap();
        let password = String::from_utf8(request.password).unwrap();
        if let Some(_) = self.database.get_game(&id) {
            let answer = ServerResponse::InvalidRequest(super::messages::Reason::IdInUse);
            let _ = stream.write(&answer.to_bytes().unwrap().as_slice());
        }else{

        } 
    }
    fn try_join_game(&self, request: JoinSessionRequest, mut stream: TcpStream) {
        let id = String::from_utf8(request.id).unwrap();
        let request_password = String::from_utf8(request.password).unwrap();
        if let Some(password) = self.database.get_game(&id) {
            if password == request_password {
                let answer = ServerResponse::Ok();
            }else{
                let answer = ServerResponse::InvalidRequest(super::messages::Reason::InvalidPassword);
            }
        }else{
            let answer = ServerResponse::InvalidRequest(super::messages::Reason::IdDoesntExist);
            let _ = stream.write(answer.to_bytes().unwrap().as_slice());
        }
    }
}
