// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::sync::Arc;

use bincode_example::{HelloReply, HelloRequest, METHOD_GREETER_SAY_HELLO};
use grpcio::{ChannelBuilder, EnvBuilder, Marshaller, MethodType};

#[derive(Clone)]
pub struct GreeterClient {
    client: grpcio::Client,
}

impl GreeterClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GreeterClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn say_hello_opt(
        &self,
        req: &HelloRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<HelloReply> {
        self.client.unary_call(&METHOD_GREETER_SAY_HELLO, req, opt)
    }

    pub fn say_hello(&self, req: &HelloRequest) -> ::grpcio::Result<HelloReply> {
        self.say_hello_opt(req, ::grpcio::CallOption::default())
    }

    pub fn say_hello_async_opt(
        &self,
        req: &HelloRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<HelloReply>> {
        self.client
            .unary_call_async(&METHOD_GREETER_SAY_HELLO, req, opt)
    }

    pub fn say_hello_async(
        &self,
        req: &HelloRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<HelloReply>> {
        self.say_hello_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Output = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}

pub trait Greeter {
    fn say_hello(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: HelloRequest,
        sink: ::grpcio::UnarySink<HelloReply>,
    );
}

pub fn create_greeter<S: Greeter + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_GREETER_SAY_HELLO, move |ctx, req, resp| {
        instance.say_hello(ctx, req, resp)
    });
    builder.build()
}

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = GreeterClient::new(ch);
    println!("Greeter server connected");

    let mut req = HelloRequest::default();
    loop {
        let reply = client.say_hello(&req).expect("rpc");
        println!("Greeter received: {:?}", reply);
    }
}
