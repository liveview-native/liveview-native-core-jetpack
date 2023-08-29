use std::hint::unreachable_unchecked;

#[cfg(target_os = "android")]
use android_logger::Config;
use cranelift_entity::EntityRef;
use jni::{
    objects::{JClass, JIntArray, JObject, JObjectArray, JString, JValue},
    sys::{jbyte, jint, jlong, jsize},
    JNIEnv,
};
use liveview_native_core::{
    diff,
    diff::PatchResult,
    dom,
    dom::{Document, NodeRef},
    ffi::{Attribute, AttributeVec, ChangeType, Element, Node, NodeData, NodeType, RustStr},
};
#[cfg(target_os = "android")]
use log::LevelFilter;

pub struct JavaResult {
    /// Raw pointer to Document
    pub val: jlong,
    pub error_msg: String,
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Document_drop(
    mut env: JNIEnv,
    _: JClass,
    // non-null raw pointer to Document
    this: jlong,
) {
    let this = this as *mut Document;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Document::drop called with `this` as null pointer",
        )
        .unwrap();
    } else {
        let _doc = Box::from_raw(this);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_empty(
    _env: JNIEnv,
    _: JClass,
    // raw pointer to new, empty Document
) -> jlong {
    let doc = Box::new(Document::empty());
    Box::into_raw(doc) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_00024Companion_initialize_1log<
    'local,
>(
    _env: JNIEnv<'local>,
    _: JClass<'local>,
) {
    #[cfg(target_os = "android")]
    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Trace)
            .with_tag("RustLog"),
    );
    log_panics::init();
    log::error!("Logging initialised from Rust");
}

/// Returns raw pointer to JavaResult
#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_00024Companion_do_1parse<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    text: JString<'local>,
    // raw pointer to JavaResult
) -> jlong {
    let text: String = env.get_string(&text).unwrap().into();
    let result = match Document::parse(text) {
        Ok(doc) => {
            let doc = Box::new(doc);
            JavaResult {
                val: Box::into_raw(doc) as jlong,
                error_msg: String::new(),
            }
        }
        Err(err) => JavaResult {
            val: 0,
            error_msg: err.to_string(),
        },
    };
    let result = Box::new(result);
    Box::into_raw(result) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_JavaResult_drop<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to JavaResult
    this: jlong,
) {
    let this = this as *mut JavaResult;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "JavaResult::drop called with `this` as null pointer",
        )
        .unwrap();
    } else {
        let _result = Box::from_raw(this);
    }
}

/// Returns raw pointer to Document else 0
#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_JavaResult_get_1val<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to JavaResult wrapping *mut Document
    this: jlong,
    // Raw pointer to Document
) -> jlong {
    let this = this as *mut JavaResult;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "JavaResult::get_val called with `this` as null pointer",
        )
        .unwrap();

        0
    } else {
        let result = &mut *this;
        result.val as jlong
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_JavaResult_get_1error<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to JavaResult
    this: jlong,
) -> JString<'local> {
    let this = this as *mut JavaResult;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "JavaResult::get_error called with `this` as null pointer",
        )
        .unwrap();

        JObject::null().into()
    } else {
        let result = &*this;

        env.new_string(result.error_msg.as_str())
            .unwrap_or_else(|_| JObject::null().into())
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Document_do_1to_1string<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to Document
    this: jlong,
) -> JString<'local> {
    let this = this as *mut Document;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Document::do_to_string call with `this` as null pointer",
        )
        .unwrap();

        JObject::null().into()
    } else {
        let doc = &*this;

        env.new_string(doc.to_string())
            .unwrap_or_else(|_| JObject::null().into())
    }
}

// Java side should ensure only u32 is passed as the node parameter
#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Document_node_1to_1string<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to Document
    this: jlong,
    // NodeRef is the distinct u32 key mapped to a Node (and not a Node raw pointer)
    node_ref: jint,
) -> JString<'local> {
    let this = this as *mut Document;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Document::node_to_string called with `this` as null pointer",
        )
        .unwrap();

        JObject::null().into()
    } else {
        let node_ref = NodeRef::new(node_ref as usize);
        let doc = &*this;
        let mut buf = String::new();

        if let Err(err) = doc.print_node(node_ref, &mut buf, dom::PrintOptions::Pretty) {
            let message = format!("Document::node_to_string failed: {:?}", err);
            env.throw_new("java/lang/RuntimeException", message)
                .unwrap();

            JObject::null().into()
        } else {
            env.new_string(buf)
                .unwrap_or_else(|_| JObject::null().into())
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Document_root<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to Document
    this: jlong,
    // NodeRef of root
) -> jint {
    let this = this as *mut Document;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Document::root called with `this` as null pointer",
        )
        .unwrap();

        0
    } else {
        let doc = &*this;
        doc.root().as_u32() as jint
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1node<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to Document
    this: jlong,
    // NodeRef is the distinct u32 key mapped to a Node (and not a Node raw pointer)
    node_ref: jint,
    // Raw pointer to Node
) -> jlong {
    let this = this as *mut Document;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Document::get_node called with `this` as null pointer",
        )
        .unwrap();

        0
    } else {
        let node = NodeRef::new(node_ref as usize);
        let doc = &*this;
        let node = Box::new(from_node(&doc, node));
        Box::into_raw(node) as jlong
    }
}

fn from_node(doc: &Document, node: NodeRef) -> Node<'_> {
    match doc.get(node) {
        dom::Node::Root => Node {
            ty: NodeType::Root,
            data: NodeData { root: () },
        },
        dom::Node::Leaf(ref s) => Node {
            ty: NodeType::Leaf,
            data: NodeData {
                leaf: RustStr::from_str(s.as_str()),
            },
        },
        dom::Node::Element(ref elem) => {
            let attrs = elem.attributes();
            let mut attributes = Vec::with_capacity(attrs.len());
            for attr in attrs {
                attributes.push(from_attr(attr));
            }
            Node {
                ty: NodeType::Element,
                data: NodeData {
                    element: Element {
                        namespace: elem
                            .name
                            .namespace
                            .map(|ns| RustStr::from_str(ns.as_str()))
                            .unwrap_or_default(),
                        tag: RustStr::from_str(elem.name.name.as_str()),
                        attributes: AttributeVec::from_vec(attributes),
                    },
                },
            }
        }
    }
}

fn from_attr(attr: &dom::Attribute) -> Attribute {
    Attribute {
        namespace: attr
            .name
            .namespace
            .map(|ns| RustStr::from_str(ns.as_str()))
            .unwrap_or_default(),
        name: RustStr::from_str(attr.name.name.as_str()),
        value: attr
            .value
            .as_str()
            .map(|v| RustStr::from_str(v))
            .unwrap_or_default(),
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1node_1leaf_1string<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to Document
    this: jlong,
    // NodeRef is the distinct u32 key mapped to a Node (and not a Node raw pointer)
    node_ref: jint,
) -> JString<'local> {
    let this = this as *mut Document;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Document::get_node_leaf_string called with `this` as null pointer",
        )
        .unwrap();

        JObject::null().into()
    } else {
        let node = NodeRef::new(node_ref as usize);
        let doc = &*this;

        match doc.get(node) {
            dom::Node::Leaf(ref s) => env
                .new_string(s.as_str())
                .unwrap_or_else(|_| JObject::null().into()),
            _ => JObject::null().into(),
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1node_1type(
    mut env: JNIEnv,
    _: JClass,
    // non-null raw pointer to Node
    node: jlong,
) -> jbyte {
    let node = node as *mut Node;

    if node.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Document::get_node_type called with `node` as null pointer",
        )
        .unwrap();

        0
    } else {
        let node = &*node;
        node.ty as jbyte
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1node_1element(
    mut env: JNIEnv,
    _: JClass,
    // non-null raw pointer to node
    node: jlong,
    // Raw pointer to Element
) -> jlong {
    let node = node as *mut Node;

    if node.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Document::get_node_element called with `node` as null pointer",
        )
        .unwrap();

        0
    } else {
        let node = &*node;
        let element = Box::new(node.data.element.clone());

        Box::into_raw(element) as jlong
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Node_00024Element_drop(
    mut env: JNIEnv,
    _: JClass,
    // non-null raw pointer to Element
    this: jlong,
) {
    let this = this as *mut Element;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Node.Element::drop called with null pointer",
        )
        .unwrap();
    } else {
        let _element = Box::from_raw(this);
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Node_00024Element_get_1namespace<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to Element
    this: jlong,
) -> JString<'local> {
    let this = this as *mut Element;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Document::get_node_element_namespace called with null pointer",
        )
        .unwrap();

        JObject::null().into()
    } else {
        let element = &*this;
        env.new_string(element.namespace.to_str())
            .unwrap_or_else(|_| JObject::null().into())
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Node_00024Element_get_1tag<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to Element
    this: jlong,
) -> JString<'local> {
    let this = this as *mut Element;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Document::get_node_element_tag called with null pointer",
        )
        .unwrap();

        JObject::null().into()
    } else {
        let element = &*this;
        env.new_string(element.tag.to_str())
            .unwrap_or_else(|_| JObject::null().into())
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Node_00024Element_get_1attributes<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to Element
    this: jlong,
) -> JObjectArray<'local> {
    let this = this as *mut Element;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Document::get_node_element_attributes called with null pointer",
        )
        .unwrap();

        JObject::null().into()
    } else {
        let element = &*this;

        let mut attributes = element.attributes.to_vec();

        let attribute_class = env
            .find_class("org/phoenixframework/liveview/lib/Attribute")
            .expect("No such class");

        let array = env
            .new_object_array(attributes.len() as jsize, &attribute_class, JObject::null())
            .expect("unable to create array");

        for (i, obj) in attributes.drain(..).enumerate() {
            let obj = Box::new(obj);
            let obj = Box::into_raw(obj) as jlong;
            let java_object = env.alloc_object(&attribute_class).unwrap();
            env.set_field(&java_object, "nativeObject", "J", JValue::from(obj))
                .expect("unable to set nativeObject");
            env.set_object_array_element(&array, i as jsize, &java_object)
                .unwrap();
        }

        array
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Attribute_drop(
    mut env: JNIEnv,
    _: JClass,
    // non-null raw pointer to Attribute
    this: jlong,
) {
    let this = this as *mut Attribute;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Attribute::drop called with `this` as null pointer",
        )
        .unwrap();
    } else {
        let _attr = Box::from_raw(this);
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Attribute_get_1name<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to Attribute
    this: jlong,
) -> JString<'local> {
    let this = this as *mut Attribute;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Attribute::get_name called with null pointer",
        )
        .unwrap();

        JObject::null().into()
    } else {
        let attr = &*(this as *mut Attribute);
        env.new_string(attr.name.to_str())
            .unwrap_or_else(|_| JObject::null().into())
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Attribute_get_1namespace<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to Attribute
    this: jlong,
) -> JString<'local> {
    let this = this as *mut Attribute;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Attribute::get_namespace called with null pointer",
        )
        .unwrap();

        JObject::null().into()
    } else {
        let attr = &*this;
        env.new_string(attr.namespace.to_str())
            .unwrap_or_else(|_| JObject::null().into())
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Attribute_get_1value<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw-pointer to Attribute
    this: jlong,
) -> JString<'local> {
    let this = this as *mut Attribute;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Attribute::get_value called with null pointer",
        )
        .unwrap();

        JObject::null().into()
    } else {
        let attr = &*this;
        env.new_string(attr.value.to_str())
            .unwrap_or_else(|_| JObject::null().into())
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1children<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to Document
    this: jlong,
    // NodeRef is the distinct u32 key mapped to a Node (and not a Node raw pointer)
    node_ref: jint,
) -> JIntArray<'local> {
    let this = this as *mut Document;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Document::get_children called with null pointer",
        )
        .unwrap();

        JObject::null().into()
    } else {
        let node = NodeRef::new(node_ref as usize);
        let doc = &*this;
        let children = doc.children(node);
        let buff = env.new_int_array(children.len() as jsize).unwrap();
        let children: Vec<jint> = children.into_iter().map(|n| n.as_u32() as jint).collect();
        env.set_int_array_region(&buff, 0, &children).unwrap();
        buff
    }
}

// Java side should ensure only u32 is passed as the node parameter
// Note! this function returns -1 when there's no parent
#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1parent(
    mut env: JNIEnv,
    _: JClass,
    // non-null raw pointer to Document
    this: jlong,
    // NodeRef is the distinct u32 key mapped to a Node (and not a Node raw pointer)
    node_ref: jint,
) -> jint {
    let this = this as *mut Document;

    if this.is_null() {
        env.throw_new(
            "java/lang/NullPointerException",
            "Document::get_parent called with null pointer",
        )
        .unwrap();

        -1
    } else {
        let node = NodeRef::new(node_ref as usize);
        let doc = &*this;

        match doc.parent(node) {
            Some(parent) => parent.as_u32() as jint,
            None => -1,
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_phoenixframework_liveview_lib_Document_merge<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    // non-null raw pointer to Document
    this: jlong,
    // non-null raw pointer to other Document
    other: jlong,
    // Callback handle interface
    interface: JObject<'local>,
) {
    let this = this as *mut Document;
    let other = other as *mut Document;

    if this.is_null() || other.is_null() {
        let message = match (this.is_null(), other.is_null()) {
            (true, true) => "Documment::merge called with `this` and `other` as null pointers",
            (true, false) => "Document::merge called with `this` as null pointer",
            (false, true) => "Document::merge called with `other` as null pointer",
            (false, false) => unreachable_unchecked(),
        };

        env.throw_new("java/lang/NullPointerException", message)
            .unwrap();

        return;
    }

    let doc = &mut *this;
    let other = &*other;

    let patches = diff::diff(doc, other);

    if patches.is_empty() {
        return;
    }

    let mut editor = doc.edit();
    let mut stack = vec![];

    for patch in patches.into_iter() {
        match patch.apply(&mut editor, &mut stack) {
            None => (),
            Some(PatchResult::Add { node, parent }) => {
                env.call_method(
                    &interface,
                    "mOnHandle",
                    "(JBII)V",
                    &[
                        JValue::Long(this as jlong),
                        JValue::Byte(ChangeType::Add as jbyte),
                        JValue::Int(node.as_u32() as jint),
                        JValue::Int(parent.as_u32() as jint),
                    ],
                )
                .unwrap();
            }
            Some(PatchResult::Remove { node, parent }) => {
                env.call_method(
                    &interface,
                    "mOnHandle",
                    "(JBII)V",
                    &[
                        JValue::Long(this as jlong),
                        JValue::Byte(ChangeType::Remove as jbyte),
                        JValue::Int(node.as_u32() as jint),
                        JValue::Int(parent.as_u32() as jint),
                    ],
                )
                .unwrap();
            }
            Some(PatchResult::Change { node }) => {
                env.call_method(
                    &interface,
                    "mOnHandle",
                    "(JBII)V",
                    &[
                        JValue::Long(this as jlong),
                        JValue::Byte(ChangeType::Change as jbyte),
                        JValue::Int(node.as_u32() as jint),
                        JValue::Int(0),
                    ],
                )
                .unwrap();
            }
            Some(PatchResult::Replace { node, parent }) => {
                env.call_method(
                    &interface,
                    "mOnHandle",
                    "(JBII)V",
                    &[
                        JValue::Long(this as jlong),
                        JValue::Byte(ChangeType::Replace as jbyte),
                        JValue::Int(node.as_u32() as jint),
                        JValue::Int(parent.as_u32() as jint),
                    ],
                )
                .unwrap();
            }
        }
    }

    editor.finish();
}
