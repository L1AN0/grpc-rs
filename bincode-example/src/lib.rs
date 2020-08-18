use grpcio::{Marshaller, MethodType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct HelloRequest {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct HelloReply {}

pub const METHOD_GREETER_SAY_HELLO: grpcio::Method<HelloRequest, HelloReply> = grpcio::Method {
    ty: MethodType::Unary,
    name: "hello",
    req_mar: Marshaller {
        ser: grpcio::bi_ser,
        de: grpcio::bi_de,
    },
    resp_mar: Marshaller {
        ser: grpcio::bi_ser,
        de: grpcio::bi_de,
    },
};

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
