use android_logger::Config;
use cranelift_entity::EntityRef;
use jni::objects::{JClass, JIntArray, JObject, JObjectArray, JString, JValue};
use jni::sys::{jbyte, jint, jlong, jsize};
use jni::JNIEnv;
use liveview_native_core::dom::NodeRef;
use liveview_native_core::ffi::{
    Attribute, AttributeVec, ChangeType, Element, Node, NodeData, NodeType, RustStr,
};
use liveview_native_core::{diff, dom};
use log::LevelFilter;

pub struct JavaResult {
    pub val: jlong,
    pub error_msg: String,
}

#[cfg(target_pointer_width = "32")]
pub fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    (val as u32) as *mut T
}

#[cfg(target_pointer_width = "64")]
pub fn jlong_to_pointer<T>(val: jlong) -> *mut T {
    val as *mut T
}

pub fn from_std_string_jstring<S: AsRef<str>>(x: S, env: JNIEnv) -> JString {
    env.new_string(x).unwrap()
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_drop(
    _env: JNIEnv,
    _: JClass,
    this: jlong,
) {
    unsafe {
        let dom: *mut dom::Document = jlong_to_pointer::<dom::Document>(this).as_mut().unwrap();
        let dom = Box::from_raw(dom);
        drop(dom);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_empty(
    _env: JNIEnv,
    _: JClass,
) -> jlong {
    let document = Box::new(dom::Document::empty());
    let raw = Box::into_raw(document);
    raw as jlong
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

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_00024Companion_do_1parse<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    text: JString<'local>,
) -> jlong {
    let text: String = env.get_string(&text).unwrap().into();
    let result = match dom::Document::parse(text) {
        Ok(doc) => {
            let doc = Box::new(doc);
            JavaResult {
                val: Box::into_raw(doc) as jlong,
                error_msg: "".into(),
            }
        }
        Err(err) => JavaResult {
            val: 0,
            error_msg: err.to_string(),
        },
    };
    let ret = Box::new(result);
    Box::into_raw(ret) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_JavaResult_drop<'local>(
    _env: JNIEnv<'local>,
    _: JClass<'local>,
    j_result: jlong,
) {
    unsafe {
        let java_result: *mut JavaResult =
            jlong_to_pointer::<JavaResult>(j_result).as_mut().unwrap();
        let java_result = Box::from_raw(java_result);
        drop(java_result);
    }
}

// returns pointer to document else 0
#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_JavaResult_get_1val<'local>(
    _env: JNIEnv<'local>,
    _: JClass<'local>,
    j_result: jlong,
) -> jlong {
    unsafe {
        let java_result: &mut JavaResult =
            jlong_to_pointer::<JavaResult>(j_result).as_mut().unwrap();
        java_result.val
    }
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_JavaResult_get_1error<'local>(
    env: JNIEnv<'local>,
    _: JClass<'local>,
    j_result: jlong,
) -> JString<'local> {
    unsafe {
        let java_result: &JavaResult = jlong_to_pointer::<JavaResult>(j_result).as_mut().unwrap();
        from_std_string_jstring(&java_result.error_msg, env)
    }
}

// takes dom
#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_do_1to_1string<'local>(
    env: JNIEnv<'local>,
    _: JClass<'local>,
    dom: jlong,
) -> JString<'local> {
    unsafe {
        let dom: &dom::Document = jlong_to_pointer::<dom::Document>(dom).as_mut().unwrap();
        from_std_string_jstring(dom.to_string(), env)
    }
}

// Java side should ensure only u32 is passed as the node parameter
#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_node_1to_1string<'local>(
    env: JNIEnv<'local>,
    _: JClass<'local>,
    this: jlong,
    node: jint,
) -> JString<'local> {
    // only u32 should be passed as node
    let node = u32::try_from(node).expect("value beyond `u32` range");
    let node = NodeRef::new(node as usize);
    let dom = unsafe { jlong_to_pointer::<dom::Document>(this).as_ref().unwrap() };
    let mut buf = String::new();
    dom.print_node(node, &mut buf, dom::PrintOptions::Pretty)
        .expect("error printing node");

    from_std_string_jstring(buf, env)
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_root(
    _env: JNIEnv,
    _: JClass,
    this: jlong,
) -> jint {
    unsafe {
        let dom: &dom::Document = jlong_to_pointer::<dom::Document>(this).as_ref().unwrap();
        dom.root().as_u32() as jint
    }
}

// Java side should ensure only u32 is passed as the node parameter
#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1node(
    _env: JNIEnv,
    _: JClass,
    this: jlong,
    node_ref: jint,
) -> jlong {
    // only u32 should be passed as node
    let node = u32::try_from(node_ref).expect("value beyond `u32` range");
    let node = NodeRef::new(node as usize);
    let dom = unsafe { jlong_to_pointer::<dom::Document>(this).as_ref().unwrap() };
    let node = Box::new(from_node(&dom, node));
    Box::into_raw(node) as jlong
}

fn from_node(doc: &dom::Document, node: NodeRef) -> Node {
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

// Java side should ensure only u32 is passed as the node parameter
#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1node_1leaf_1string<
    'local,
>(
    env: JNIEnv<'local>,
    _: JClass<'local>,
    this: jlong,
    node_ref: jint,
) -> JString<'local> {
    // only u32 should be passed as node
    let node = u32::try_from(node_ref).expect("value beyond `u32` range");
    let node = NodeRef::new(node as usize);
    let doc = unsafe { jlong_to_pointer::<dom::Document>(this).as_ref().unwrap() };
    match doc.get(node) {
        dom::Node::Leaf(ref s) => from_std_string_jstring(s.to_string(), env),
        _ => {
            panic!("node isn't a leaf")
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1node_1type(
    _env: JNIEnv,
    _: JClass,
    node: jlong,
) -> jbyte {
    unsafe {
        let node: &Node = jlong_to_pointer::<Node>(node).as_ref().unwrap();
        node.ty as jbyte
    }
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1node_1element(
    _env: JNIEnv,
    _: JClass,
    node: jlong,
) -> jlong {
    unsafe {
        let node: *mut Node = jlong_to_pointer::<Node>(node).as_mut().unwrap();
        let ret = Box::new((*node).data.element);
        Box::into_raw(ret) as jlong as jlong
    }
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Node_00024Element_drop(
    _env: JNIEnv,
    _: JClass,
    this: jlong,
) {
    unsafe {
        let dom: *mut Element = jlong_to_pointer::<Element>(this).as_mut().unwrap();
        let dom = Box::from_raw(dom);
        drop(dom);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1node_1element_1namespace<
    'local,
>(
    env: JNIEnv<'local>,
    _: JClass<'local>,
    element: jlong,
) -> JString<'local> {
    unsafe {
        let element: &Element = jlong_to_pointer::<Element>(element).as_mut().unwrap();
        from_std_string_jstring(String::from(element.namespace.to_str()), env)
    }
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1node_1element_1tag<
    'local,
>(
    env: JNIEnv<'local>,
    _: JClass<'local>,
    element: jlong,
) -> JString<'local> {
    let element: &Element = unsafe { jlong_to_pointer::<Element>(element).as_mut().unwrap() };
    from_std_string_jstring(String::from(element.tag.to_str()), env)
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1node_1element_1attributes<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    element: jlong,
) -> JObjectArray<'local> {
    let element: &Element = unsafe { jlong_to_pointer::<Element>(element).as_mut().unwrap() };
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

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Attribute_drop(
    _env: JNIEnv,
    _: JClass,
    this: jlong,
) {
    unsafe {
        let dom: *mut Attribute = jlong_to_pointer::<Attribute>(this).as_mut().unwrap();
        let dom = Box::from_raw(dom);
        drop(dom);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Attribute_get_1name<'local>(
    env: JNIEnv<'local>,
    _: JClass<'local>,
    attr: jlong,
) -> JString<'local> {
    let attribute: &Attribute = unsafe { jlong_to_pointer::<Attribute>(attr).as_mut().unwrap() };
    from_std_string_jstring(String::from(attribute.name.to_str()), env)
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Attribute_get_1namespace<'local>(
    env: JNIEnv<'local>,
    _: JClass<'local>,
    attr: jlong,
) -> JString<'local> {
    let attribute: &Attribute = unsafe { jlong_to_pointer::<Attribute>(attr).as_ref().unwrap() };
    from_std_string_jstring(String::from(attribute.namespace.to_str()), env)
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Attribute_get_1value<'local>(
    env: JNIEnv<'local>,
    _: JClass<'local>,
    attr: jlong,
) -> JString<'local> {
    let attribute: &Attribute = unsafe { jlong_to_pointer::<Attribute>(attr).as_ref().unwrap() };
    from_std_string_jstring(String::from(attribute.value.to_str()), env)
}

// Java side should ensure only u32 is passed as the node_ref parameter
#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1children<'local>(
    env: JNIEnv<'local>,
    _: JClass<'local>,
    this: jlong,
    node_ref: jint,
) -> JIntArray<'local> {
    // only u32 should be passed as node
    let node = u32::try_from(node_ref).expect("value beyond `u32` range");
    let node = NodeRef::new(node as usize);
    let dom = unsafe { jlong_to_pointer::<dom::Document>(this).as_ref().unwrap() };
    let children = dom.children(node);
    let java_array = env.new_int_array(children.len() as jsize).unwrap();
    let children: Vec<jint> = children.into_iter().map(|n| n.as_u32() as jint).collect();
    env.set_int_array_region(&java_array, 0, &children).unwrap();
    java_array
}

// Java side should ensure only u32 is passed as the node parameter
// Note! this function returns -1 when there's no parent
#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_get_1parent(
    _env: JNIEnv,
    _: JClass,
    this: jlong,
    node_ref: jint,
) -> jint {
    // only u32 should be passed as node
    let node = u32::try_from(node_ref).expect("value beyond `u32` range");
    let node = NodeRef::new(node as usize);
    let dom = unsafe { jlong_to_pointer::<dom::Document>(this).as_ref().unwrap() };

    match dom.parent(node) {
        Some(parent) => parent.as_u32() as jint,
        None => -1,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_phoenixframework_liveview_lib_Document_merge<'local>(
    mut env: JNIEnv<'local>,
    _: JClass<'local>,
    this: jlong,
    other: jlong,
    interface: JObject<'local>,
) {
    let doc = unsafe { jlong_to_pointer::<dom::Document>(this).as_mut().unwrap() };
    let other = unsafe { jlong_to_pointer::<dom::Document>(other).as_ref().unwrap() };
    let mut patches = crate::diff::diff(doc, other);
    if patches.is_empty() {
        return;
    }
    let mut editor = doc.edit();
    let mut stack = vec![];
    for patch in patches.drain(..) {
        let patch_result = patch.apply(&mut editor, &mut stack);
        match patch_result {
            None => (),
            Some(diff::PatchResult::Add { node, parent }) => {
                env.call_method(
                    &interface,
                    "mOnHandle",
                    "(JBII)V",
                    &[
                        JValue::Long(this),
                        JValue::Byte(ChangeType::Add as jbyte),
                        JValue::Int(node.as_u32() as jint),
                        JValue::Int(parent.as_u32() as jint),
                    ],
                )
                .unwrap();
            }
            Some(diff::PatchResult::Remove { node, parent }) => {
                env.call_method(
                    &interface,
                    "mOnHandle",
                    "(JBII)V",
                    &[
                        JValue::Long(this),
                        JValue::Byte(ChangeType::Remove as jbyte),
                        JValue::Int(node.as_u32() as jint),
                        JValue::Int(parent.as_u32() as jint),
                    ],
                )
                .unwrap();
            }
            Some(diff::PatchResult::Change { node }) => {
                env.call_method(
                    &interface,
                    "mOnHandle",
                    "(JBII)V",
                    &[
                        JValue::Long(this),
                        JValue::Byte(ChangeType::Change as jbyte),
                        JValue::Int(node.as_u32() as jint),
                        JValue::Int(0),
                    ],
                ).unwrap();
            }
        }
    }
    editor.finish();
}
