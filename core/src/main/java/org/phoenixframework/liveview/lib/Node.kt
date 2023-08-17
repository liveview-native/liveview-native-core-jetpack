package org.phoenixframework.liveview.lib

/** This class represents the valid node types of a `Document` tree */
sealed class Node {

    /**
     * A marker node that indicates the root of a document A document may only have a single root,
     * and it has no attributes
     */
    object Root : Node()

    /** A typed node that can carry attributes and may contain other nodes */
    class Element: Node {
        private var nativeObject: Long

        internal constructor(pointer: Long) {
            nativeObject = pointer
        }

        fun namespace(): String {
            return get_namespace(nativeObject)
        }

        fun tag(): String {
            return get_tag(nativeObject)
        }

        fun attributes(): Array<Attribute> {
            return get_attributes(nativeObject)
        }

        private external fun get_attributes(element: Long): Array<Attribute>

        private external fun get_tag(element: Long): String
    
        private external fun get_namespace(element: Long): String    

        private external fun drop(pointer: Long)

        @Synchronized
        private fun delete() {
            if (nativeObject != 0L) {
                drop(nativeObject)
                nativeObject = 0
            }
        }

        @Throws(Throwable::class)
        protected fun finalize() {
            delete()
        }
    }

    /**
     * A leaf node is an untyped node, typically text, and does not have any attributes or children
     */
    data class Leaf(val value: String) : Node()
}
