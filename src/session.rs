use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use bit_field::BitField;

//use {crate::db::types};

struct Session{
    id: String,
    usr: u32,
    scopes: u8
}

impl Session{
    // Initalize a new self
    fn new() -> Session{
        return Session{
            id: thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .collect(),
            usr: 0,
            scopes: 0
        }
    }
    fn set_user(&mut self, _id: u32){
        self.usr = _id;
    }
    fn set_privl(&mut self, _scope: u8){
        self.scopes = _scope;
    }
    fn get_userid(&self) -> u32{
        return self.usr;
    }
    //TODO: Get User obj from ID
    //fn get_user(&self) -> db::types::User{
    //    
    //}
}

/* Datareq scopes, global
userinf = Basic user info (id, username, surname, first name, image)
sens_userinf = Sensitive user info (Personal data)
perf_userinf = Performance related info (Grades)
abs_userinf = Absence related info
*/


struct DataScopes{
    userinf_read: bool,
    userinf_write: bool,
    sensinf_read: bool,
    sensinf_write: bool,
    perfinf_read: bool,
    perfinf_write: bool,
    abs_usrinf_read: bool,
    abs_usrinf_write: bool
}

impl DataScopes{
    fn new() -> DataScopes{
        return DataScopes{
            userinf_read: false,
            userinf_write: false,
            sensinf_read: false,
            sensinf_write: false,
            perfinf_read: false,
            perfinf_write: false,
            abs_usrinf_read: false,
            abs_usrinf_write: false
        }
    }
    fn from_bitfield(_bf: u8) -> DataScopes{
        return DataScopes{
            userinf_read: _bf.get_bit(0),
            userinf_write: _bf.get_bit(1),
            sensinf_read: _bf.get_bit(2),
            sensinf_write: _bf.get_bit(3),
            perfinf_read: _bf.get_bit(4),
            perfinf_write: _bf.get_bit(5),
            abs_usrinf_read: _bf.get_bit(6),
            abs_usrinf_write: _bf.get_bit(7)
        }
    }
    fn to_bitfield(&self) -> u8{
        let mut i: u8 = 0;
        i.set_bit(0, self.userinf_read);
        i.set_bit(1, self.userinf_write);
        i.set_bit(2, self.sensinf_read);
        i.set_bit(3, self.sensinf_write);
        i.set_bit(4, self.perfinf_read);
        i.set_bit(5, self.perfinf_write);
        i.set_bit(6, self.abs_usrinf_read);
        i.set_bit(7, self.abs_usrinf_write);
        return i;
    }
}





