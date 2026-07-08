---
name: translate-character-sheet-tasks
description: Task list for character sheet translation tool implementation
status: draft
author: "Claude"
created: "2026-07-08"
---

# Translation Tool Implementation Tasks

## Overview

Implementation of a Python translation tool for character sheet markdown files.

## Status: ✅ COMPLETED

All core functionality has been implemented and tested.

## Completed Tasks

### Phase 1: Translation Dictionary ✓

- [x] Create `translations.yaml` with all required translations (156 entries)
- [x] Populate missing translations from Alaric.md analysis
- [x] Verify number format conversion rules

### Phase 2: Python Script Implementation ✓

- [x] Create `translate_character_sheet.py`
- [x] Implement YAML parsing
- [x] Implement number format conversion (English → German)
- [x] Implement text translation logic
- [x] Add CLI argument handling

### Phase 3: Testing ✓

- [x] Test with Alaric.md as input
- [x] Verify number format conversion works correctly
- [x] Verify core translations applied correctly
- [x] Compare output with expected German format

## Translation Gaps Found & Addressed

The following terms from Alaric.md were NOT in the original translations.md and have been added:

### Section Headers (missing from translations.md)
- Status, Primary Attributes, Derived Attributes, Spell List, Skill List, Attunements, Bone Glyphs, Bonds, Weapons, Equipment, Character Traits, Titles

### Attributes (missing from translations.md)
- Health Regen, Mana Regen, Stamina Regen, Basic Damage Melee, Basic Damage Ranged, Basic Damage Magic, Basic Damage Soul

### Spells (missing from translations.md)
- Shadow Nova, Shadow Decoy, Shadow Switch, Nether Shroud

### Skills (missing from translations.md)
- Shadow Infusion, Shadow Decoy, Shadow Switch, Nether Shroud, Equinox Reaver

### Equipment (missing from translations.md)
- The Eidolon Stylus, Dwarven Boots of Ass-Kicking, Dwarven Spellslinger Pants, Ultimate Dwarven Leather Jacket of Readiness, Eye of the Tiger

### Traits (missing from translations.md)
- Omniscient Perception, Omnilingual, Natural Immunity against Poison, Natural Immunity against Acid, Natural Immunity against Mental Manipulation

### Titles (missing from translations.md)
- We haven't even started yet, Death Wish, Scrappy Survivor, Master of my Mind, System Exploiter, Codebreaker, Seeker of Truths, Titan's Bane, Soul of the Party, Who's the bully now?

## Number Format Conversions (Implemented)

| English Format | German Format | Status |
|----------------|---------------|--------|
| 2,042 (thousands) | 2.042 | ✅ Working |
| 68.07 (decimal) | 68,07 | ✅ Working |
| 22,803.30 (both) | 22.803,30 | ✅ Working |
| 1,500 Mana | 1.500 Mana | ✅ Working |

## Files Created

| File | Status |
|------|--------|
| `translations.yaml` | ✅ Created (156 translations) |
| `translate_character_sheet.py` | ✅ Created |
| `openspec/changes/translate-character-sheet/proposal.md` | ✅ Created |
| `openspec/changes/translate-character-sheet/design.md` | ✅ Created |

## Remaining Considerations

Some descriptive phrases remain in English because:
1. They are context-dependent descriptions
2. Word-by-word translation would break the grammar
3. These can be added to translations.yaml as full phrases when needed

Example: `might damage close targets`, `creates 5 auto-aiming darts`, etc.

## Usage

```bash
python3 translate_character_sheet.py <input.md> <translations.yaml> <output.md>
```

Example:
```bash
python3 translate_character_sheet.py Alaric.md translations.yaml Alaric_de.md
```