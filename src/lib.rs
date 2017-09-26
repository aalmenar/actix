#![feature(default_type_parameter_fallback)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;
extern crate rand;

extern crate bytes;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate tokio_io;
extern crate tokio_core;

mod actor;
mod arbiter;
mod address;
mod sync_address;
mod builder;
mod context;
mod message;
mod sink;
mod framed;
mod system;
mod utils;

pub mod fut;
pub mod prelude;

pub use actor::{Actor, MessageHandler, StreamHandler};
pub use address::{Address, SyncAddress, Subscriber, AsyncSubscriber};
pub use arbiter::{Arbiter, StopArbiter, ArbiterAddress};
pub use builder::ActorBuilder;
pub use context::{Context, ActixFutureSpawner};
pub use framed::{ActixFramed, ActixFramedRead, ActixFramedWrite};
pub use message::{MessageResult, MessageFuture, MessageFutureResult, MessageFutureError};
pub use sink::Sink;
pub use system::{System, SystemExit};
pub use utils::Condition;