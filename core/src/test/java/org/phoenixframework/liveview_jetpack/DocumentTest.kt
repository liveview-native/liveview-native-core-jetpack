

import org.junit.Test
import org.junit.Assert.*
import org.phoenixframework.liveview.lib.Document

class DocumentTest {
    @Test
    fun it_constructs_empty_native_doc() {
        Document()
    }

    @Test
    fun it_morphs_live_form() {
        var doc = Document.parse("""
        <VStack modifiers="">
            <VStack>
                <LiveForm id="login" phx-submit="login">
                    <TextField name="email" modifiers="">
                        Email
                    </TextField>
                    <LiveSubmitButton modifiers="">
                        <Text>Enter</Text>
                    </LiveSubmitButton>
                </LiveForm>
            </VStack>
        </VStack>
        """);

        var to = Document.parse("""
        <VStack modifiers="">
            <VStack>
                <Text>Success! Check your email for magic link</Text>
            </VStack>
        </VStack>
        """);

        doc.merge(to,  Document.Companion.Handler());
    }

    @Test
    fun merge_json_into_document_exception() {
        var doc = Document.parse("""
        <VStack modifiers="">
            <VStack>
                <LiveForm id="login" phx-submit="login">
                    <TextField name="email" modifiers="">
                        Email
                    </TextField>
                    <LiveSubmitButton modifiers="">
                        <Text>Enter</Text>
                    </LiveSubmitButton>
                </LiveForm>
            </VStack>
        </VStack>
        """);
        var invalid_json = """
        <VStack modifiers="">
            <VStack>
                <Text>Success! Check your email for magic link</Text>
            </VStack>
        </VStack>
        """;

        assertThrows(RuntimeException::class.java) {
            doc.mergeFragmentJson(invalid_json,  Document.Companion.Handler())
        }

    }

    @Test
    fun merge_json_into_document() {
        var doc = Document.parse("""
        <VStack modifiers="">
            <VStack>
                <LiveForm id="login" phx-submit="login">
                    <TextField name="email" modifiers="">
                        Email
                    </TextField>
                    <LiveSubmitButton modifiers="">
                        <Text>Enter</Text>
                    </LiveSubmitButton>
                </LiveForm>
            </VStack>
        </VStack>
        """);

        var valid_json = """
        {
            "0": {
                "0": "<Text>Success! Check your email for magic link</Text>",
                "s": [
                    "<VStack>",
                    "</VStack>"
                ]
            },
            "s": [
                "<VStack modifiers>",
                "</VStack>"
            ]
        }
        """;

        doc.mergeFragmentJson(valid_json,  Document.Companion.Handler())

    }
}
