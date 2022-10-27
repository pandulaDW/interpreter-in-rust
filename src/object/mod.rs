mod objects;

pub enum ObjectType {
    Integer,
    Boolean,
    Null,
}

pub trait Object {
    fn object_type() -> ObjectType;
    fn inspect(&self) -> String;
}
