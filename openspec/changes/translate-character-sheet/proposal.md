---
name: translate-character-sheet
description: Python translation tool for character sheet markdown files
status: draft
author: "Claude"
created: "2026-07-08"
---

# Translation Tool for Character Sheets

## Summary

Create a Python-based translation tool that converts English character sheet markdown files to German, including number format conversion from English (1,234.56) to German (1.234,56) format.

## Motivation

- The `Alaric.md` file exists in English with English number formatting
- A partial `translations.md` file exists with some translations
- Need a reusable tool to translate any character sheet file
- Number formats must be adjusted during translation (comma ↔ period swap)

## Specification

### Input Files

1. **Character Sheet** (`.md` file) - English text with English number formatting
2. **Translation Dictionary** (`.yaml` file) - Key-value pairs mapping English to German

### Output File

A translated character sheet with:
- All English text replaced by German translations
- Number formats converted (e.g., `2,042/3,000` → `2.042/3.000`)
- Same markdown structure and formatting preserved

### Translation Dictionary Format (YAML)

```yaml
# translations.yaml
version: 1.0
source_language: en
target_language: de

# Number format conversion settings
number_format:
  thousands_separator: "."
  decimal_separator: ","

# Section Headers
sections:
  Status: Status
  Primary Attributes: Primärattribute
  Derived Attributes: Abgeleitete Attribute
  Spell List: Spruchliste
  Skill List: Fähigkeitsliste
  Attunements: Synchronisationen
  Bone Glyphs: Knochen-Siegel
  Bonds: Bindungen
  Weapons: Waffen
  Equipment: Ausrüstung
  Character Traits: Charakter-Vorzüge
  Titles: Titel

# Attributes
attributes:
  Body: Körper
  Mind: Verstand
  Spirit: Geist
  Soul: Seele
  Health: Gesundheit
  Mana: Mana
  Stamina: Ausdauer
  Physical Resistance: Physischer Schutz
  Mental Resilience: Metaphysischer Schutz
  Basic Damage Melee: Grundschaden Nahkampf
  Basic Damage Ranged: Grundschaden Fernkampf
  Basic Damage Magic: Grundschaden Magie
  Basic Damage Soul: Grundschaden Seele
  Health Regen: Gesundheitsregeneration
  Mana Regen: Manaregeneration
  Stamina Regen: Ausdauerregeneration

# Classes
classes:
  Eclipsed Initiate: Verfinsterter Novize

# Professions
professions:
  Bone Engraver: Knochen-Graveur
  Aetheric Engineer: Ätheringenieur

# Attunement Titles
attunement_titles:
  Pioneer of Twilight: Pionier des Zwielichts
  Paragon of Hope: Paragon der Hoffnung
  Harbinger of Dread: Vorbote des Schreckens

# Spells
spells:
  Umbral Judgement: Umbrales Urteil
  Umbral Volley: Umbralhagel
  Shadow Nova: Schattennova
  Umbral Chains: Umbralketten
  Call of the Shadows: Schattenruf
  Healing Light: Heilendes Licht
  Healing Touch: Heilende Berührung
  Umbral Lance: Umbral-Lanze

# Skills
skills:
  Phantom Reclamation: Rückgewinnung des Schattens
  Umbral Journey: Umbrale Reise
  Shadow Infusion: Schatteninfusion
  Shadow Decoy: Schattenklon
  Shadow Switch: Schattentausch
  Nether Shroud: Jenseitsschleier
  Equinox Reaver: Schnitter der Tagundnachtgleichen

# Equipment
equipment:
  The Eidolon Stylus: Der Eidolon Stift
  Dwarven Boots of Ass-Kicking: Zwergen-Kampfstiefel des Arschtritts
  Dwarven Spellslinger Pants: Zwergische Zauberer-Hosen
  Ultimate Dwarven Leather Jacket of Readiness: Ultimative Zwergen-Lederjacke der Bereitschaft
  Eye of the Tiger: Auge des Tigers

# Traits
traits:
  Omniscient Perception: Allwissende Wahrnehmung
  Omnilingual: Allsprachiges Verständnis
  Natural Immunity against Poison: Natürliche Immunität gegen Vergiftung
  Natural Immunity against Acid: Natürliche Immunität gegen Säure
  Natural Immunity against Mental Manipulation: Natürliche Immunität gegen geistige Manipulation

# Titles
titles:
  We haven't even started yet: Wir sind noch nicht angefangen
  Death Wish: Todeswunsch
  Scrappy Survivor: Zäher Überlebender
  Master of my Mind: Meister meines Geistes
  System Exploiter: Systemausbeuter
  Codebreaker: Codezerbrecher
  Seeker of Truths: Wahrheitsforscher
  Titan's Bane: Titanenzerstörer
  Soul of the Party: Seele der Gemeinschaft
  Who's the bully now?: Wer ist jetzt der Bully?

# Keywords
keywords:
  Damage: Schadens
  Healing: Heilung
  Points: Punkte
  Percent: Prozent
  Seconds: Sekunden
  Minutes: Minuten
  instant: sofort
  passive: passiv
  Duration: Dauer
```

### Python Script Architecture

```
translate_character_sheet.py
├── load_translations()     # Parse YAML translation file
├── convert_number_format() # English ↔ German number format conversion
├── translate_text()        # Apply translations to text
├── process_file()          # Main processing pipeline
└── main()                  # CLI entry point
```

### Number Format Conversion Logic

```python
def convert_number_format(text: str) -> str:
    """
    Convert English number format to German format.
    
    Examples:
        2,042/3,000     → 2.042/3.000
        68.07 %         → 68,07 %
        22,803.30       → 22.803,30
        1,500 Mana      → 1.500 Mana
    """
```

### Algorithm for Number Conversion

1. **Identify numbers with comma-thousands and optional decimal**
   - Pattern: `\d{1,3}(,\d{3})*(\.\d+)?`
   
2. **Conversion rules:**
   - If number contains both `,` and `.` → swap both separators
   - If number contains only `,` → replace with `.`
   - If number contains only `.` → replace with `,`

3. **Special handling:**
   - Preserve numbers in URLs, version strings (e.g., `7.5 minutes` → `7,5 Minuten`)
   - Don't modify numbers already in German format

### CLI Usage

```bash
python translate_character_sheet.py <input.md> <translations.yaml> <output.md>
```

Example:
```bash
python translate_character_sheet.py Alaric.md translations.yaml Alaric_de.md
```

## Implementation Plan

### Phase 1: Create Translation Dictionary

1. Create `translations.yaml` with all required translations
2. Populate missing translations from Alaric.md analysis
3. Verify number format conversion rules

### Phase 2: Implement Python Script

1. Create `translate_character_sheet.py`
2. Implement YAML parsing
3. Implement number format conversion
4. Implement text translation logic
5. Add CLI argument handling

### Phase 3: Testing

1. Test with Alaric.md as input
2. Verify number format conversion
3. Verify all translations applied correctly
4. Compare output with expected German format

## Files to Create

| File | Purpose |
|------|---------|
| `translations.yaml` | Translation dictionary in YAML format |
| `translate_character_sheet.py` | Main Python translation script |

## Risks & Considerations

- **Word boundaries**: Need to handle partial word matches (e.g., "Spirit" in "Spirit Regen")
- **Number format edge cases**: Numbers in version strings, measurements, etc.
- **Markdown syntax preservation**: Don't translate markdown syntax (`[text]{.u}`, `**text**`, etc.)
- **Order of translations**: More specific translations should be applied before general ones

## Alternative Approaches Considered

1. **JSON format** - Rejected for YAML (more readable, supports comments)
2. **Inline translations in script** - Rejected for maintainability
3. **Machine translation API** - Rejected for simplicity and offline capability

## Success Criteria

- [ ] `translations.yaml` contains all 100+ required translations
- [ ] Python script runs without errors
- [ ] Number formats are correctly converted
- [ ] All English text is translated to German
- [ ] Markdown structure is preserved
- [ ] Output is valid markdown