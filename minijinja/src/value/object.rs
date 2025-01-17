use std::any::Any;
use std::fmt;

use crate::error::{Error, ErrorKind};
use crate::value::Value;
use crate::vm::State;

/// A utility trait that represents a dynamic object.
///
/// The engine uses the [`Value`] type to represent values that the engine
/// knows about.  Most of these values are primitives such as integers, strings
/// or maps.  However it is also possible to expose custom types without
/// undergoing a serialization step to the engine.  For this to work a type
/// needs to implement the [`Object`] trait and be wrapped in a value with
/// [`Value::from_object`](crate::value::Value::from_object). The ownership of
/// the object will then move into the value type.
//
/// The engine uses reference counted objects with interior mutability in the
/// value type.  This means that all trait methods take `&self` and types like
/// [`Mutex`](std::sync::Mutex) need to be used to enable mutability.
//
/// Objects need to implement [`Display`](std::fmt::Display) which is used by
/// the engine to convert the object into a string if needed.  Additionally
/// [`Debug`](std::fmt::Debug) is required as well.
pub trait Object: fmt::Display + fmt::Debug + Any + Sync + Send {
    /// Invoked by the engine to get the attribute of an object.
    ///
    /// Where possible it's a good idea for this to align with the return value
    /// of [`attributes`](Self::attributes) but it's not necessary.
    ///
    /// If an attribute does not exist, `None` shall be returned.
    ///
    /// A note should be made here on side effects: unlike calling objects or
    /// calling methods on objects, accessing attributes is not supposed to
    /// have side effects.  Neither does this API get access to the interpreter
    /// [`State`] nor is there a channel to send out failures as only an option
    /// can be returned.  If you do plan on doing something in attribute access
    /// that is fallible, instead use a method call.
    fn get_attr(&self, name: &str) -> Option<Value> {
        let _name = name;
        None
    }

    /// An enumeration of attributes that are known to exist on this object.
    ///
    /// The default implementation returns an empty iterator.  If it's not possible
    /// to implement this, it's fine for the implementation to be omitted.  The
    /// enumeration here is used by the `for` loop to iterate over the attributes
    /// on the value.
    fn attributes(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        Box::new(None.into_iter())
    }

    /// Called when the engine tries to call a method on the object.
    ///
    /// It's the responsibility of the implementer to ensure that an
    /// error is generated if an invalid method is invoked.
    ///
    /// To convert the arguments into arguments use the
    /// [`from_args`](crate::value::from_args) function.
    fn call_method(&self, state: &State, name: &str, args: &[Value]) -> Result<Value, Error> {
        let _state = state;
        let _args = args;
        Err(Error::new(
            ErrorKind::InvalidOperation,
            format!("object has no method named {}", name),
        ))
    }

    /// Called when the object is invoked directly.
    ///
    /// The default implementation just generates an error that the object
    /// cannot be invoked.
    ///
    /// To convert the arguments into arguments use the
    /// [`from_args`](crate::value::from_args) function.
    fn call(&self, state: &State, args: &[Value]) -> Result<Value, Error> {
        let _state = state;
        let _args = args;
        Err(Error::new(
            ErrorKind::InvalidOperation,
            "tried to call non callable object",
        ))
    }
}

impl<T: Object> Object for std::sync::Arc<T> {
    fn get_attr(&self, name: &str) -> Option<Value> {
        T::get_attr(self, name)
    }

    fn attributes(&self) -> Box<dyn Iterator<Item = &str> + '_> {
        T::attributes(self)
    }

    fn call_method(&self, state: &State, name: &str, args: &[Value]) -> Result<Value, Error> {
        T::call_method(self, state, name, args)
    }

    fn call(&self, state: &State, args: &[Value]) -> Result<Value, Error> {
        T::call(self, state, args)
    }
}
