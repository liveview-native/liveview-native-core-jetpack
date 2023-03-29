package org.phoenixframework.liveview.lib

class Attribute(private var nativeObject: Long) {

    val name: String by lazy { get_name(nativeObject) }

    /** The namespace of an attribute */
    val namespace by lazy { get_namespace(nativeObject) }

    val value by lazy { get_value(nativeObject) }

    private external fun get_name(pointer: Long): String

    private external fun get_value(pointer: Long): String

    private external fun get_namespace(pointer: Long): String

    override fun toString(): String =
        "Attribute {\n" +
        "  Name: ${name.ifEmpty { "None" }}\n" +
        "  Namespace: ${namespace.ifEmpty { "None" }}\n" +
        "  Value: ${value.ifEmpty { "None" }}\n" +
        "}"

    @Synchronized
    private fun delete() {
        if (nativeObject != 0L) {
            drop(nativeObject)
            nativeObject = 0
        }
    }

    @Throws(Throwable::class)
    protected fun finalize() {
        try {
            delete()
        } finally {
            // do nothing
        }
    }

    private external fun drop(pointer: Long)
}
