use clap::Parser;
use protobuf::Message;

#[derive(Parser)]
struct Args {
    #[clap(long, alias("my_name"))]
    my_name: Option<std::string::String>,
    #[clap(long, alias("my_bar"))]
    my_bar: Option<i32>,
}

pub fn set_protos() -> messages_rust_proto::Foo {
    let args: Args;
    match Args::try_parse() {
        Ok(r) => args = r,
        Err(e) => {
            panic!("{}", e)
        }
    }

    let mut out = messages_rust_proto::Foo::new();
    match args.my_name {
        Some(name) => out.set_name(name),
        None => {}
    }
    match args.my_bar {
        Some(my_bar) => {
            let mut bar = messages_rust_proto::Bar::new();
            bar.set_i(my_bar);
            let mut bar_any = protobuf::well_known_types::Any {
                type_url: bar.descriptor().full_name().to_string(),
                value: bar.write_to_bytes().unwrap(),
                unknown_fields: bar.unknown_fields,
                cached_size: bar.cached_size,
            };
            out.set_generic(bar_any);
        }
        None => {}
    }
    out
}
