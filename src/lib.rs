#![feature(test)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
extern crate test;
extern crate bincode;
extern crate serde_cbor;

#[macro_use]
extern crate serde_json;

use std::sync::Arc;
use test::Bencher;
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use std::borrow::Cow;
use bincode::{serialize, deserialize, Infinite};
use serde_cbor::{to_vec, from_slice, ser};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NodeId {
    pub name: String,
    pub addr: String
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pid {
    pub group: Option<String>,
    pub name: String,
    pub node: NodeId
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PidV2 {
    pub group: Option<String>,
    pub name: String,
    pub node: NodeId,
    #[serde(default)]
    pub owner: Option<String>
}


#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Msg {
    NodeId(NodeId),
    Pid(Pid)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Msg2 {
    NodeId(NodeId),
    Pid(Pid),
    Pid2(PidV2)
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
fn serialize_pid_msgpack(b: &mut Bencher) {
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
fn serialize_pid_json(b: &mut Bencher) {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    b.iter(|| serde_json::to_string(&pid))
}

#[bench]
fn serialize_pid_cbor(b: &mut Bencher) {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    b.iter(|| serde_cbor::to_vec(&pid))
}

#[bench]
fn serialize_pid_cbor_packed(b: &mut Bencher) {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    b.iter(|| serde_cbor::ser::to_vec_packed(&pid))
}

#[bench]
fn serialize_pid_bincode() {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    b.iter(|| serialize(&pid, Infinite).unwrap())
}

#[test]
fn deserialize_old_version_bincode() {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };

    let encoded = serialize(&pid, Infinite).unwrap();
    let pid_v2: PidV2 = deserialize(&encoded[..]).unwrap();
    println!("{:?}", pid_v2);
}

#[test]
fn deserialize_old_version_json() {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };

    let j = serde_json::to_string(&pid).unwrap();
    let pid_v2: PidV2 = serde_json::from_str(&j).unwrap();
    println!("{:?}", pid_v2);
}

#[test]
fn deserialize_old_version_cbor() {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };

    let j = serde_cbor::to_vec(&pid).unwrap();
    let pid_v2: PidV2 = serde_cbor::from_slice(&j).unwrap();
}

#[test]
fn deserialize_old_version_cbor_packed() {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };

    let j = serde_cbor::ser::to_vec_packed(&pid).unwrap();
    let pid_v2: PidV2 = serde_cbor::from_slice(&j).unwrap();
}

#[test]
fn deserialize_old_version_msgpack() {
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
    let mut de = Deserializer::new(&buf[..]);
    let pid_v2: PidV2 = Deserialize::deserialize(&mut de).unwrap();
}

#[bench]
fn serialize_arc_pid_msgpack(b: &mut Bencher) {
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
fn serialize_cow_pid_msgpack(b: &mut Bencher) {
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
fn deserialize_pid_msgpack(b: &mut Bencher) {
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
fn deserialize_pid_json(b: &mut Bencher) {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    let j = serde_json::to_string(&pid).unwrap();
    b.iter(|| serde_json::from_str::<Pid>(&j))
}

#[bench]
fn deserialize_pid_cbor(b: &mut Bencher) {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    let j = serde_cbor::to_vec(&pid).unwrap();
    b.iter(|| serde_cbor::from_slice::<Pid>(&j))
}

#[bench]
fn deserialize_pid_cbor_packed(b: &mut Bencher) {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    let j = serde_cbor::ser::to_vec_packed(&pid).unwrap();
    b.iter(|| serde_cbor::from_slice::<Pid>(&j))
}


#[bench]
fn deserialize_pid_bincode(b: &mut Bencher) {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    let v: Vec<u8> = serialize(&pid, Infinite).unwrap();
    b.iter(|| {
        let env: Pid = deserialize(&v[..]).unwrap();
        env
    })
}

#[bench]
fn deserialize_pid_borrowed_bincode(b: &mut Bencher) {
    let node = NodeIdBorrowed {
        name: "node1",
        addr: "127.0.0.1:5000"
    };
    let pid = PidBorrowed {
        group: Some("haret"),
        name: "r1",
        node: node
    };
    let v: Vec<u8> = serialize(&pid, Infinite).unwrap();
    b.iter(|| {
        let env: PidBorrowed = deserialize(&v[..]).unwrap();
        env
    })
}

#[bench]
fn deserialize_old_enum_into_new_bincode(b: &mut Bencher) {
    let node = NodeId {
        name: "node1".to_owned(),
        addr: "127.0.0.1:5000".to_owned()
    };
    let pid = Pid {
        group: Some("haret".to_owned()),
        name: "r1".to_owned(),
        node: node
    };
    let msg = Msg::Pid(pid);
    let v: Vec<u8> = serialize(&msg, Infinite).unwrap();
    b.iter(|| {
        let msg: Msg2 = deserialize(&v[..]).unwrap();
        msg
    })
}

#[bench]
fn serialize_pid_borrowed_msgpack(b: &mut Bencher) {
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
fn deserialize_pid_borrowed_msgpack(b: &mut Bencher) {
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
fn deserialize_pid_borrowed_json(b: &mut Bencher) {
    let node = NodeIdBorrowed {
        name: "node1",
        addr: "127.0.0.1:5000"
    };
    let pid = PidBorrowed {
        group: Some("haret"),
        name: "r1",
        node: node
    };
    let serialized = serde_json::to_string(&pid).unwrap();
    b.iter(|| serde_json::from_str::<PidBorrowed>(&serialized))
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
