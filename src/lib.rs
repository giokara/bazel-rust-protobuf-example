use clap::Parser;
#[derive(Parser)]
struct Args {
    #[clap(long, alias("my_name"))]
    my_name: Option<std::String>,
    #[clap(long, alias("my_bar"))]
    my_bar: Option<i32>,
}

pub fn set_protos() -> messages_rust_proto::Foo {
    let args: Args;
    match Flags::try_parse() {
        Ok(r) => args = r,
        Err(e) => {
            panic!(e)
        }
    }

    let out = messages_rust_proto::Foo::new();
    match args.my_name {
        Some(name) => out.set_name(name),
        None => {}
    }
    match args.my_bar {
        Some(my_bar) => {
            let mut bar_any = protobuf::well_known_types::Any::new();
            bar_any.set_type_url(String::from(
                messages_rust_proto::Bar::descriptor_static().full_name(),
            ));
            let mut bar = messages_rust_proto::Bar::new();
            bar.set_i(my_bar);
            bar_any.set_value(bar.write_to_bytes().unpack());
        }
        None => {}
    }
    out
}
