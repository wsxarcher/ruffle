//! `flash.net.FileReference` builtin/prototype

use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::object::TObject;
use crate::avm2::traits::Trait;
use crate::avm2::Multiname;
use crate::avm2::{Activation, Error, Namespace, Object, QName, Value};
use crate::display_object::DisplayObject;
use crate::display_object::TDisplayObject;
use crate::string::AvmString;
use flash_lso::types::{AMFVersion, Lso};
use gc_arena::{GcCell, MutationContext};
use ruffle_wstr::WString;
use std::borrow::Cow;

/// Implements `flash.net.FileReference`'s instance constructor.
fn instance_init<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(mut this) = this {
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
        QName::new(Namespace::package("flash.net"), "FileReference"),
        Some(Multiname::new(
            Namespace::package("flash.events"),
            "EventDispatcher",
        )),
        Method::from_builtin(instance_init, "<FileReference instance initializer>", mc),
        Method::from_builtin(class_init, "<FileReference class initializer>", mc),
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
