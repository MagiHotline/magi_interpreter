// Suppress all warnings from generated code
#![allow(warnings)]

pub mod magilexer {
    include!(concat!(env!("OUT_DIR"), "/magilexer.rs"));
}

pub mod magiparser {
    include!(concat!(env!("OUT_DIR"), "/magiparser.rs"));
}

pub mod magilistener {
    include!(concat!(env!("OUT_DIR"), "/magilistener.rs"));
}

pub mod magivisitor {
    include!(concat!(env!("OUT_DIR"), "/magivisitor.rs"));

}
