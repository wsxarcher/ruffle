//! `flash.net.FileFilter` builtin/prototype

use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::object::TObject;
use crate::avm2::Multiname;
use crate::avm2::{Activation, Error, Namespace, Object, QName, Value};
use crate::string::AvmString;
use gc_arena::{GcCell, MutationContext};
use ruffle_wstr::WString;

/// Implements `flash.net.FileFilter`'s instance constructor.
fn instance_init<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(mut this) = this {
        activation.super_init(this, &[])?;

        let description = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_string(activation)?;

        let extension = args
            .get(1)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_string(activation)?;

        let mac_type = args
            .get(2)
            .cloned()
            .unwrap_or(Value::Null)
            .coerce_to_string(activation)?;

        this.set_property(
            &Multiname::public("description"),
            description.into(),
            activation,
        )?;
        this.set_property(
            &Multiname::public("extension"),
            extension.into(),
            activation,
        )?;
        this.set_property(&Multiname::public("macType"), mac_type.into(), activation)?;
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

/// Implements `FileFilter.description`'s getter.
pub fn description<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(_this) = this {
        let gc = activation.context.gc_context;
        return Ok(Value::String(AvmString::new(gc, WString::new())));
    }

    Ok(Value::Undefined)
}

/// Implements `FileFilter.description`'s setter.
pub fn set_description<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(mut _this) = this {
        let _description = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_string(activation)?;
    }

    Ok(Value::Undefined)
}

/// Implements `FileFilter.extension`'s getter.
pub fn extension<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(_this) = this {
        let gc = activation.context.gc_context;
        return Ok(Value::String(AvmString::new(gc, WString::new())));
    }

    Ok(Value::Undefined)
}

/// Implements `FileFilter.extension`'s setter.
pub fn set_extension<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(mut _this) = this {
        let _extension = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_string(activation)?;
    }

    Ok(Value::Undefined)
}

/// Implements `FileFilter.macType`'s getter.
pub fn mac_type<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(_this) = this {
        let gc = activation.context.gc_context;
        return Ok(Value::String(AvmString::new(gc, WString::new())));
    }

    Ok(Value::Undefined)
}

/// Implements `FileFilter.macType`'s setter.
pub fn set_mac_type<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(mut _this) = this {
        let _mac_type = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_string(activation)?;
    }

    Ok(Value::Undefined)
}

pub fn create_class<'gc>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.net"), "FileFilter"),
        Some(Multiname::new(Namespace::package(""), "Object")),
        Method::from_builtin(instance_init, "<FileFilter instance initializer>", mc),
        Method::from_builtin(class_init, "<FileFilter class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);
    write.set_attributes(ClassAttributes::SEALED);

    const PUBLIC_INSTANCE_PROPERTIES: &[(
        &str,
        Option<NativeMethodImpl>,
        Option<NativeMethodImpl>,
    )] = &[
        ("description", Some(description), Some(set_description)),
        ("extension", Some(extension), Some(set_extension)),
        ("macType", Some(mac_type), Some(set_mac_type)),
    ];
    write.define_public_builtin_instance_properties(mc, PUBLIC_INSTANCE_PROPERTIES);

    class
}
