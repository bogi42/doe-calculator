#!/usr/bin/env python3
"""
Character Sheet Translator

Translates English markdown character sheets to German with number format conversion.
Usage: python translate_character_sheet.py <input.md> <translations.yaml> <output.md>

Example:
    python translate_character_sheet.py Alaric.md translations.yaml Alaric_de.md
"""

import re
import sys
import argparse
from pathlib import Path
from typing import Dict, List, Tuple


def load_translations(path: str) -> Dict[str, str]:
    """
    Load translations from YAML file.
    Supports simple YAML format with - key: value pairs.
    """
    translations = {}

    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()

    # Parse YAML - simple format: - key: value
    for line in content.split('\n'):
        line = line.strip()
        # Skip empty lines, comments, and section headers
        if not line or line.startswith('#') or line.startswith('version:') or \
           line.startswith('source_language:') or line.startswith('target_language:') or \
           line.startswith('number_format:') or line.startswith('thousands_separator:') or \
           line.startswith('decimal_separator:') or line.startswith('---'):
            continue

        # Parse translation entries: - key: value
        if ':' in line and line.startswith('- '):
            # Remove the leading "- "
            line = line[2:]
            # Split on first colon
            parts = line.split(':', 1)
            if len(parts) == 2:
                key = parts[0].strip()
                value = parts[1].strip()
                if key and value:
                    translations[key] = value

    return translations


def convert_english_number_to_german(number_str: str) -> str:
    """
    Convert an English number format to German format.

    English format:
        - Thousands separator: comma (,)
        - Decimal separator: period (.)

    German format:
        - Thousands separator: period (.)
        - Decimal separator: comma (,)

    Examples:
        2,042 → 2.042
        68.07 → 68,07
        22,803.30 → 22.803,30
    """
    if ',' in number_str and '.' in number_str:
        # Has both comma (thousands) and period (decimal): 22,803.30 → 22.803,30
        return number_str.replace(',', 'X').replace('.', ',').replace('X', '.')
    elif ',' in number_str:
        # Only comma (thousands separator): 2,042 → 2.042
        return number_str.replace(',', '.')
    elif '.' in number_str:
        # Only period (decimal separator): 68.07 → 68,07
        return number_str.replace('.', ',')
    else:
        return number_str


def convert_number_format(text: str) -> str:
    """
    Convert all English number formats in text to German format.

    Matches numbers like:
    - 2,042 (thousands)
    - 68.07 (decimal)
    - 22,803.30 (both)
    - 2,042/3,000 (compound with /)
    """
    # Pattern to match numbers with comma-thousands and optional decimal part
    # This pattern handles numbers adjacent to separators like / or -
    def replace_number(match):
        num = match.group(0)
        return convert_english_number_to_german(num)

    # Match numbers with thousands separators (commas) and optional decimals
    # Pattern: 1-3 digits, then (comma + 3 digits)*, then optional (period + digits)
    pattern = r'\d{1,3}(?:,\d{3})*(?:\.\d+)?'

    return re.sub(pattern, replace_number, text)


def translate_text(text: str, translations: Dict[str, str]) -> str:
    """
    Apply translations to text using phrase-based replacement.

    Sorts translations by key length (longest first) to prevent partial matches.
    Uses word boundaries for safety, but handles special characters like parentheses.
    """
    # Sort translations by key length (longest first) to prevent partial matches
    sorted_translations = sorted(translations.items(), key=lambda x: len(x[0]), reverse=True)

    result = text
    for english, german in sorted_translations:
        # Escape special regex characters
        escaped = re.escape(english)

        # Determine if we should use word boundaries
        # Don't use word boundaries if the text starts or ends with non-word characters
        if english and (not english[0].isalnum() or not english[-1].isalnum()):
            # Text starts or ends with non-word char, don't use \b
            pattern = escaped
        else:
            # Use word boundaries for safety
            pattern = r'\b' + escaped + r'\b'

        result = re.sub(pattern, german, result)

    return result


def process_file(input_path: str, translation_path: str, output_path: str) -> None:
    """
    Main processing function.

    Reads input file, applies translations, converts number formats,
    and writes the result to output file.
    """
    # Load translations
    translations = load_translations(translation_path)

    if not translations:
        print(f"Error: No translations loaded from {translation_path}")
        sys.exit(1)

    print(f"Loaded {len(translations)} translations from {translation_path}")

    # Read input file
    with open(input_path, 'r', encoding='utf-8') as f:
        content = f.read()

    print(f"Processing {input_path}...")

    # Translate text
    translated = translate_text(content, translations)

    # Convert number formats
    converted = convert_number_format(translated)

    # Write output
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(converted)

    print(f"Output written to {output_path}")


def main():
    """CLI entry point."""
    parser = argparse.ArgumentParser(
        description='Translate character sheet from English to German',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog='''
Examples:
  %(prog)s Alaric.md translations.yaml Alaric_de.md
  %(prog)s input.md translations.yaml output.md
        '''
    )
    parser.add_argument('input', help='Input markdown file (English)')
    parser.add_argument('translations', help='Translation dictionary YAML file')
    parser.add_argument('output', help='Output translated file (German)')

    args = parser.parse_args()

    # Verify input file exists
    if not Path(args.input).exists():
        print(f"Error: Input file not found: {args.input}")
        sys.exit(1)

    # Verify translation file exists
    if not Path(args.translations).exists():
        print(f"Error: Translation file not found: {args.translations}")
        sys.exit(1)

    process_file(args.input, args.translations, args.output)


if __name__ == '__main__':
    main()