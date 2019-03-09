use crate::math::point::Point3f;

pub enum Primitive {
    Emitter(Emitter),
    Receiver(Receiver),
}

pub struct Emitter {
    transform: Point3f,
}

pub struct Receiver {
    transform: Point3f,
}