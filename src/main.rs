extern crate capnp;
extern crate capnpc;

mod cats_capnp {
    include!("./schema/cats_capnp.rs");
}

pub mod CatsInShelter {
    use crate::cats_capnp::{cat,cats_in_shelter};
    use capnp::serialize_packed;
    pub fn WriteCatsInShelter() -> ::capnp::Result<()> {
        let mut message = ::capnp::message::Builder::new_default();
        {
            let list_of_cats = message.init_root::<cats_in_shelter::Builder>();

            let mut cat = list_of_cats.init_catt(1);
            {
            let mut sherlock = cat.reborrow().get(0);
            sherlock.set_id(1);
            sherlock.set_name("Sherlock");
            sherlock.set_breed("bengal");
            
            
            let mut sherlock_owner = sherlock.reborrow().init_owners(1);
            sherlock_owner.reborrow().get(0).set_owner_name("Nikita");
            sherlock_owner.reborrow().get(0).set_age(16);
            }
        
    }
        serialize_packed::write_message(&mut ::std::io::stdout(), &message)
    }
}

fn main() {

    let args : Vec<String> = ::std::env::args().collect();
    if args.len() < 2 {
        println!("usage: $ {} [write || read]", args[0]);
    } else {
        match &*args[1] {
            "write" => CatsInShelter::WriteCatsInShelter().unwrap(),
            _ => {println!("unrecognized argument") }
        }
    }
    
}
