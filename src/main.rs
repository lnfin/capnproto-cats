extern crate capnp;
extern crate capnpc;

mod cats_capnp {
    include!("./schema/cats_capnp.rs");
}

pub mod cats_at_home {
    use crate::cats_capnp::{cat,cats_at_home};
    use capnp::serialize_packed;
    pub fn write_cats_at_home() -> ::capnp::Result<()> {
        let mut message = ::capnp::message::Builder::new_default();
        {
            let list_of_cats = message.init_root::<cats_at_home::Builder>();

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

    cats_at_home::write_cats_at_home().unwrap();

}
