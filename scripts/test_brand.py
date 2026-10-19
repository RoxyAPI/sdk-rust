from pathlib import Path
import unittest

from brand import violations


class BrandTests(unittest.TestCase):
    def assert_typography(self, path, content, line):
        self.assertIn((line, "typography"), list(violations(Path(path), content)))

    def test_python_docstrings_and_inline_comments_are_scanned(self):
        text = "Two" + chr(0x2014) + "words"
        self.assert_typography("sample.py", f'"""{text}"""', 1)
        self.assert_typography("sample.py", f'value = 1  # {text}', 1)
        self.assert_typography("sample.py", f'def sample():\n    """{text}"""\n    pass', 2)

    def test_rust_inline_and_nested_block_comments_are_scanned(self):
        text = "Two" + chr(0x2014) + "words"
        self.assert_typography("sample.rs", f'fn sample() {{}} // {text}', 1)
        self.assert_typography("sample.rs", f'/* outer\n/* inner */\n{text}\n*/', 3)
        self.assert_typography("sample.rs", "let quote = '\"'; // " + text + '\nlet next = "normal";', 1)

    def test_code_strings_remain_code(self):
        text = "Two" + chr(0x2014) + "words"
        snippets = {
            "sample.py": f'value = "{text}"\nheader = "X-API-Key"',
            "sample.rs": f'let value = "{text}"; let raw = r##"// {text}"##;',
            "sample.sh": f'value="{text}" # Normal comment',
        }
        for path, content in snippets.items():
            with self.subTest(path=path):
                self.assertEqual(list(violations(Path(path), content)), [])

    def test_comments_in_markdown_code_fences_are_scanned(self):
        text = "Two" + chr(0x2014) + "words"
        self.assert_typography("README.md", f'Example:\n\n```rust\nlet value = 1; // {text}\n```', 4)
        self.assert_typography("README.md", f'~~~python\n"""{text}"""\n~~~', 2)
        self.assert_typography("README.md", f'```rust\n// {text}', 2)
        self.assertEqual(list(violations(Path("README.md"), f'```rust\nlet value = "{text}";\n```')), [])

    def test_plain_prose_and_citation_metadata_are_scanned(self):
        text = "Two" + chr(0x2014) + "words"
        self.assert_typography("README.md", text, 1)
        self.assert_typography("CITATION.cff", f'abstract: "{text}"', 1)


if __name__ == "__main__":
    unittest.main()
