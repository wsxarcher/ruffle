//! `flash.net.FileReferenceList` builtin/prototype

use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::Multiname;
use crate::avm2::{Activation, Error, Namespace, Object, QName, Value};
use gc_arena::{GcCell, MutationContext};

/// Implements `flash.net.FileReferenceList`'s instance constructor.
fn instance_init<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(this) = this {
        activation.super_init(this, &[])?;
    }

    Ok(Value::Undefined)
}

fn class_init<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    Ok(Value::Undefined)
}

pub fn create_class<'gc>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.net"), "FileReferenceList"),
        Some(Multiname::new(
            Namespace::package("flash.events"),
            "EventDispatcher",
        )),
        Method::from_builtin(
            instance_init,
            "<FileReferenceList instance initializer>",
            mc,
        ),
        Method::from_builtin(class_init, "<FileReferenceList class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);
    write.set_attributes(ClassAttributes::SEALED);

    const PUBLIC_INSTANCE_PROPERTIES: &[(
        &str,
        Option<NativeMethodImpl>,
        Option<NativeMethodImpl>,
    )] = &[];
    write.define_public_builtin_instance_properties(mc, PUBLIC_INSTANCE_PROPERTIES);

    class
}
