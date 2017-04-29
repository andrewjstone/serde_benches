#![feature(test)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
extern crate test;
extern crate bincode;

use std::sync::Arc;
use test::Bencher;
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use std::borrow::Cow;
use bincode::{serialize, deserialize, Infinite};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct NodeId {
    pub name: String,
    pub addr: String
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pid {
    pub group: Option<String>,
    pub name: String,
    pub node: NodeId
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Envelope {
    pid1: Pid,
    pid2: Pid,
    pid3: Pid
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NodeIdBorrowed<'a> {
    pub name: &'a str,
    pub addr: &'a str
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PidBorrowed<'a> {
    pub group: Option<&'a str>,
    pub name: &'a str,
    pub node: NodeIdBorrowed<'a>
}

#[bench]
fn serialize_pid(b: &mut Bencher) {
    let mut buf = Vec::new();
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    b.iter(|| pid.serialize(&mut Serializer::new(&mut buf)))
}

#[bench]
fn serialize_envelope(b: &mut Bencher) {
    let mut buf = Vec::new();
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    let envelope = Envelope {
        pid1: pid.clone(),
        pid2: pid.clone(),
        pid3: pid
    };
    b.iter(|| envelope.serialize(&mut Serializer::new(&mut buf)))
}

#[bench]
fn serialize_envelope_bincode(b: &mut Bencher) {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    let envelope = Envelope {
        pid1: pid.clone(),
        pid2: pid.clone(),
        pid3: pid
    };
    b.iter(|| serialize(&envelope, Infinite).unwrap())
}

#[bench]
fn serialize_arc_pid(b: &mut Bencher) {
    let mut buf = Vec::new();
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Arc::new(Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    });
    b.iter(|| pid.serialize(&mut Serializer::new(&mut buf)))
}

#[bench]
fn serialize_cow_pid(b: &mut Bencher) {
    let mut buf = Vec::new();
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid: Cow<Pid> = Cow::Owned(Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    });
    b.iter(|| pid.serialize(&mut Serializer::new(&mut buf)))
}

#[bench]
fn deserialize_pid(b: &mut Bencher) {
    let mut buf = Vec::new();
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    pid.serialize(&mut Serializer::new(&mut buf)).unwrap();
    b.iter(|| {
        let mut de = Deserializer::new(&buf[..]);
        let pid: Result<Pid, rmps::decode::Error> = Deserialize::deserialize(&mut de);
        pid
    })
}

#[bench]
fn deserialize_envelope(b: &mut Bencher) {
    let mut buf = Vec::new();
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    let envelope = Envelope {
        pid1: pid.clone(),
        pid2: pid.clone(),
        pid3: pid
    };
    envelope.serialize(&mut Serializer::new(&mut buf)).unwrap();
    b.iter(|| {
        let mut de = Deserializer::new(&buf[..]);
        let env: Envelope = Deserialize::deserialize(&mut de).unwrap();
        env
    })
}

#[bench]
fn deserialize_envelope_bincode(b: &mut Bencher) {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    let envelope = Envelope {
        pid1: pid.clone(),
        pid2: pid.clone(),
        pid3: pid
    };
    let v: Vec<u8> = serialize(&envelope, Infinite).unwrap();
    b.iter(|| {
        let env: Envelope = deserialize(&v[..]).unwrap();
        env
    })
}

#[bench]
fn serialize_pid_borrowed(b: &mut Bencher) {
    let mut buf = Vec::new();
    let node = NodeIdBorrowed {
        name: "node1",
        addr: "127.0.0.1:5000"
    };
    let pid = PidBorrowed {
        group: Some("haret"),
        name: "r1",
        node: node
    };
    b.iter(|| pid.serialize(&mut Serializer::new(&mut buf)))
}

#[bench]
fn deserialize_pid_borrowed(b: &mut Bencher) {
    let mut buf = Vec::new();
    let node = NodeIdBorrowed {
        name: "node1",
        addr: "127.0.0.1:5000"
    };
    let pid = PidBorrowed {
        group: Some("haret"),
        name: "r1",
        node: node
    };
    pid.serialize(&mut Serializer::new(&mut buf)).unwrap();
    b.iter(|| {
        let mut de = Deserializer::new(&buf[..]);
        let pid: Result<PidBorrowed, rmps::decode::Error> = Deserialize::deserialize(&mut de);
        pid
    })
}

#[bench]
fn clone_pid(b: &mut Bencher) {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    b.iter(|| pid.clone())
}

#[bench]
fn clone_cow_pid(b: &mut Bencher) {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid: Cow<Pid> = Cow::Owned(Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    });
    b.iter(|| pid.clone())
}

#[bench]
fn clone_arc_pid(b: &mut Bencher) {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Arc::new(Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    });
    b.iter(|| pid.clone())
}

#[bench]
fn clone_pid_borrowed(b: &mut Bencher) {
    let node = NodeIdBorrowed {
        name: "node1",
        addr: "127.0.0.1:5000"
    };
    let pid = PidBorrowed {
        group: Some("haret"),
        name: "r1",
        node: node
    };
    b.iter(|| pid.clone())
}

#[bench]
fn compare_pids_equal_success(b: &mut Bencher) {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid1 = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    let pid2 = pid1.clone();
    b.iter(|| pid1 == pid2)
}

#[bench]
fn compare_pids_equal_fail(b: &mut Bencher) {
    let node1 = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let node2 = NodeId {
        name: "node2".to_owned(),
        addr: "127.0.0.1:6000".to_owned()
    };
    let pid1 = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node1
    };
    let pid2 = Pid {
        group: Some("haret".to_owned()),
        name: "r2".to_owned(),
        node: node2
    };
    b.iter(|| pid1 == pid2)
}
