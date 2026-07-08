---
name: translate-character-sheet-design
description: Detailed design for the character sheet translation tool
status: draft
author: "Claude"
created: "2026-07-08"
---

# Translation Tool Design

## Architecture Overview

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Input File     │────▶│ Translation      │────▶│  Output File    │
│  (English)      │     │  Script          │     │  (German)       │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │  translations.yaml │
                    │  (Translation Dict)│
                    └──────────────────┘
```

## Component Design

### 1. Translation Dictionary (`translations.yaml`)

Structured as a flat key-value store with categories:

```yaml
# Key format: "category.subcategory.key" or "key" for top-level
# Value: German translation

# Example structure:
attributes:
  Body: Körper
  Mind: Verstand
  Spirit: Geist
  Soul: Seele
  
# For simple lookups:
- Health: Gesundheit
- Mana: Mana
```

### 2. Number Format Converter

#### English to German Conversion Rules

| English Format | German Format | Example |
|----------------|---------------|---------|
| `1,234` (thousands) | `1.234` | `2,042` → `2.042` |
| `123.45` (decimal) | `123,45` | `68.07` → `68,07` |
| `1,234.56` (both) | `1.234,56` | `22,803.30` → `22.803,30` |

#### Regex Pattern

```python
# Match numbers with comma-thousands and optional decimal
pattern = r'\b\d{1,3}(?:,\d{3})*(?:\.\d+)?\b'
```

#### Conversion Algorithm

```python
def convert_english_to_german(number_str: str) -> str:
    if ',' in number_str and '.' in number_str:
        # Has both: 22,803.30 → 22.803,30
        return number_str.replace(',', 'TEMP').replace('.', ',').replace('TEMP', '.')
    elif ',' in number_str:
        # Only comma (thousands): 2,042 → 2.042
        return number_str.replace(',', '.')
    elif '.' in number_str:
        # Only period (decimal): 68.07 → 68,07
        return number_str.replace('.', ',')
    else:
        return number_str
```

### 3. Text Translator

#### Translation Strategy

1. **Sort translations by length (descending)** - Prevent partial matches
   - "Basic Damage Melee" before "Damage"
   - "Umbral Judgement" before "Judgement"

2. **Use word boundaries** - Prevent matching inside other words

3. **Preserve markdown syntax** - Don't translate `[text]{.u}`, `**text**`, etc.

#### Translation Algorithm

```python
def translate_text(text: str, translations: Dict[str, str]) -> str:
    # Sort by length (longest first) to prevent partial matches
    sorted_translations = sorted(translations.items(), key=lambda x: len(x[0]), reverse=True)
    
    result = text
    for english, german in sorted_translations:
        # Use word boundaries for safety
        pattern = r'\b' + re.escape(english) + r'\b'
        result = re.sub(pattern, german, result)
    
    return result
```

### 4. Main Processing Pipeline

```python
def process_file(input_path: str, translation_path: str, output_path: str):
    # 1. Load translations
    translations = load_translations(translation_path)
    
    # 2. Read input file
    with open(input_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # 3. Translate text
    translated = translate_text(content, translations)
    
    # 4. Convert number formats
    converted = convert_number_format(translated)
    
    # 5. Write output
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(converted)
```

## Data Flow Diagram

```
                    ┌─────────────────────────────────────┐
                    │           Input File (Alaric.md)    │
                    │  | [Status]{.u}                     │
                    │  | **Name:** Alaric Nachtmoor       │
                    │  | **Level/Exp:** 30 (2,042/3,000)  │
                    └─────────────────────────────────────┘
                                      │
                                      ▼
                    ┌─────────────────────────────────────┐
                    │     Translation Script              │
                    │                                     │
                    │  1. Load translations.yaml          │
                    │  2. Replace "Status" → "Status"     │
                    │  3. Replace "Health" → "Gesundheit" │
                    │  4. Replace "Mana" → "Mana"         │
                    │  5. Convert "2,042" → "2.042"       │
                    │  6. Convert "68.07 %" → "68,07 %" │
                    └─────────────────────────────────────┘
                                      │
                                      ▼
                    ┌─────────────────────────────────────┐
                    │          Output File                │
                    │  | [Status]{.u}                     │
                    │  | **Name:** Alaric Nachtmoor       │
                    │  | **Level/Exp:** 30 (2.042/3.000)  │
                    └─────────────────────────────────────┘
```

## Translation Categories

### Priority Order for Translation

1. **Section Headers** - Highest priority (appear at top of sections)
2. **Attributes** - Core game mechanics
3. **Classes/Professions** - Character identity
4. **Spells/Skills** - Abilities
5. **Equipment** - Gear names
6. **Traits/Titles** - Character features
7. **Keywords** - Generic terms used throughout

### Translation Dictionary Structure

```yaml
# Format: key: german_translation
# Comments indicate source of translation

# === SECTION HEADERS ===
- Status: Status
- Primary Attributes: Primärattribute
- Derived Attributes: Abgeleitete Attribute

# === ATTRIBUTES ===
- Body: Körper
- Mind: Verstand
- Spirit: Geist
- Soul: Seele
- Health: Gesundheit
- Mana: Mana
- Stamina: Ausdauer

# === NUMBERS & UNITS ===
- Points: Punkte
- Percent: Prozent
- Seconds: Sekunden
- Minutes: Minuten
- instant: sofort
- passive: passiv
- Duration: Dauer
- Damage: Schadens
- Healing: Heilung

# === CLASSES ===
- Eclipsed Initiate: Verfinsterter Novize

# === PROFESSIONS ===
- Bone Engraver: Knochen-Graveur
- Aetheric Engineer: Ätheringenieur

# === ATTUNEMENTS ===
- Pioneer of Twilight: Pionier des Zwielichts
- Paragon of Hope: Paragon der Hoffnung
- Harbinger of Dread: Vorbote des Schreckens

# === SPELLS ===
- Umbral Judgement: Umbrales Urteil
- Umbral Volley: Umbralhagel
- Shadow Nova: Schattennova
- Umbral Chains: Umbralketten
- Call of the Shadows: Schattenruf
- Healing Light: Heilendes Licht
- Healing Touch: Heilende Berührung
- Umbral Lance: Umbral-Lanze

# === SKILLS ===
- Phantom Reclamation: Rückgewinnung des Schattens
- Umbral Journey: Umbrale Reise
- Shadow Infusion: Schatteninfusion
- Shadow Decoy: Schattenklon
- Shadow Switch: Schattentausch
- Nether Shroud: Jenseitsschleier
- Equinox Reaver: Schnitter der Tagundnachtgleichen

# === EQUIPMENT ===
- The Eidolon Stylus: Der Eidolon Stift
- Dwarven Boots of Ass-Kicking: Zwergen-Kampfstiefel des Arschtritts
- Dwarven Spellslinger Pants: Zwergische Zauberer-Hosen
- Ultimate Dwarven Leather Jacket of Readiness: Ultimative Zwergen-Lederjacke der Bereitschaft
- Eye of the Tiger: Auge des Tigers

# === TRAITS ===
- Omniscient Perception: Allwissende Wahrnehmung
- Omnilingual: Allsprachiges Verständnis
- Natural Immunity against Poison: Natürliche Immunität gegen Vergiftung
- Natural Immunity against Acid: Natürliche Immunität gegen Säure
- Natural Immunity against Mental Manipulation: Natürliche Immunität gegen geistige Manipulation

# === TITLES ===
- We haven't even started yet: Wir sind noch nicht angefangen
- Death Wish: Todeswunsch
- Scrappy Survivor: Zäher Überlebender
- Master of my Mind: Meister meines Geistes
- System Exploiter: Systemausbeuter
- Codebreaker: Codezerbrecher
- Seeker of Truths: Wahrheitsforscher
- Titan's Bane: Titanenzerstörer
- Soul of the Party: Seele der Gemeinschaft
- Who's the bully now?: Wer ist jetzt der Bully?
```

## Edge Cases & Special Handling

### Numbers in Different Contexts

| Context | Example | Handling |
|---------|---------|----------|
| Experience | `2,042/3,000 [68.07 %]` | Both numbers converted |
| Damage | `22,803.30 Damage` | Number converted, "Damage" translated |
| Cooldown | `7.5 minutes` | Decimal converted, "minutes" translated |
| Percentage | `50.00 Percent` | Decimal converted, "Percent" translated |

### Markdown Syntax Preservation

- `[Status]{.u}` → `[Status]{.u}` (no translation - syntax)
- `**Name:**` → `**Name:**` (no translation - markdown syntax)
- `| **Body:** 601` → `| **Körper:** 601` (attribute translated)

### Order-Sensitive Translations

Must translate longer phrases first:
- "Basic Damage Melee" before "Damage Melee" before "Damage"
- "Umbral Judgement" before "Judgement"

## Testing Strategy

### Test Cases

1. **Basic number conversion**
   - Input: `2,042`
   - Expected: `2.042`

2. **Decimal conversion**
   - Input: `68.07 %`
   - Expected: `68,07 %`

3. **Combined number**
   - Input: `22,803.30 Damage`
   - Expected: `22.803,30 Schadens`

4. **Section header**
   - Input: `[Status]{.u}`
   - Expected: `[Status]{.u}`

5. **Attribute translation**
   - Input: `**Health:** 9,261/9,261`
   - Expected: `**Gesundheit:** 9.261/9.261`

6. **Full line**
   - Input: `| **Level/Exp:** 30 (2,042/3,000 [68.07 %])`
   - Expected: `| **Level/Exp:** 30 (2.042/3.000 [68,07 %])`

## Implementation Notes

### Python Dependencies

- `pyyaml` - For YAML parsing
- `re` - For regex operations (standard library)
- `argparse` - For CLI argument handling (standard library)

### Script Structure

```python
#!/usr/bin/env python3
"""
Character Sheet Translator
Translates English markdown character sheets to German with number format conversion.
"""

import re
import sys
import argparse
from pathlib import Path
from typing import Dict, List

try:
    import yaml
except ImportError:
    print("Error: PyYAML is required. Install with: pip install pyyaml")
    sys.exit(1)

def load_translations(path: str) -> Dict[str, str]:
    """Load translations from YAML file."""
    pass

def convert_number_format(text: str) -> str:
    """Convert English number format to German."""
    pass

def translate_text(text: str, translations: Dict[str, str]) -> str:
    """Apply translations to text."""
    pass

def process_file(input_path: str, translation_path: str, output_path: str) -> None:
    """Main processing function."""
    pass

def main():
    """CLI entry point."""
    parser = argparse.ArgumentParser(description='Translate character sheet')
    parser.add_argument('input', help='Input markdown file')
    parser.add_argument('translations', help='Translation dictionary YAML file')
    parser.add_argument('output', help='Output translated file')
    args = parser.parse_args()
    
    process_file(args.input, args.translations, args.output)

if __name__ == '__main__':
    main()
```