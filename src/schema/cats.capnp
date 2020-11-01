@0xf5f9d9d40ead01c6;

struct Cat {
    id @0: Int32;
    name @1: Text;
    breed  @2: Text;
    owners @3:  List(Owner);

    struct Owner {
        ownerName @0: Text;
        age @1: UInt8;
    }

}

struct CatsInShelter {
  catt @0 :List(Cat);
}