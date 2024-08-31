use axum::{extract::ws::Message, Extension};
use serde_json::Value;
use socketioxide::{extract::{Data, SocketRef}, SocketIo};

use crate::chat::schema::UserCnt;

pub async fn socket_layer() {
    let (layer, io) = SocketIo::builder().with_state(UserCnt::new()).build_layer();

    io.ns("/", |s: SocketRef| {
        s.on(
            "new message",
            |s: SocketRef, Data::<Value>(msg), Extension::<Username>(username)| {
                let msg = Res::Message {
                    username,
                    message: msg,
                };
                s.broadcast().emit("new message", msg).ok();
            },
        );

        // s.on(
        //     "add user",
        //     |s: SocketRef, Data::<String>(username), user_cnt: State<UserCnt>| {
        //         if s.extensions.get::<Username>().is_some() {
        //             return;
        //         }
        //         let num_users = user_cnt.add_user();
        //         s.extensions.insert(Username(username.clone()));
        //         s.emit("login", Res::Login { num_users }).ok();

        //         let res = Res::UserEvent {
        //             num_users,
        //             username: Username(username),
        //         };
        //         s.broadcast().emit("user joined", res).ok();
        //     },
        // );

        // s.on("typing", |s: SocketRef, Extension::<Username>(username)| {
        //     s.broadcast()
        //         .emit("typing", Res::Username { username })
        //         .ok();
        // });

        // s.on(
        //     "stop typing",
        //     |s: SocketRef, Extension::<Username>(username)| {
        //         s.broadcast()
        //             .emit("stop typing", Res::Username { username })
        //             .ok();
        //     },
        // );

        // s.on_disconnect(
        //     |s: SocketRef, user_cnt: State<UserCnt>, Extension::<Username>(username)| {
        //         let num_users = user_cnt.remove_user();
        //         let res = Res::UserEvent {
        //             num_users,
        //             username,
        //         };
        //         s.broadcast().emit("user left", res).ok();
        //     },
        // );
    });
}
