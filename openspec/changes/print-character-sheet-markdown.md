---
name: print-character-sheet-markdown
description: Change Print Character Sheet output from HTML to Markdown
status: draft
author: "Claude"
created: "2026-07-08"
---

# Print Character Sheet → Markdown Output

## Summary

Change the "Print Character Sheet" functionality to output Markdown instead of HTML. The existing HTML-formatted output will be replaced with a Markdown format that uses a pipe-prefixed line-block pattern compatible with the user's Markdown encoder.

## Motivation

- The HTML output is no longer needed
- Markdown format is more portable and readable
- The existing `Alaric.md` file demonstrates the desired output format

## Specification

### Output Format Changes

| HTML Element | Markdown Equivalent |
|--------------|---------------------|
| `<span class="u">text</span>` | `[text]{.u}` |
| `<b>text</b>` | `**text**` |
| `<i>text</i>` | `*text*` |
| `<br/>` | Removed (line breaks in Markdown suffice) |
| `╰╼(...)` | Unchanged (special character for sub-items) |
| Line prefix | `| ` (creates line-block div in encoder) |

### File Extension Change

- Current: `{character_name}.html`
- New: `{character_name}.md`

### Message Update

- Current: `HTML-Output saved to:`
- New: `Markdown-Output saved to:`

## Implementation Plan

### 1. Update `src/interactive/mod.rs`

**Location:** Lines 128-137

Change:
```rust
CharacterMenu::PrintCharacterSheet => {
    let mut outfile = mc.file_path.clone();
    outfile.set_extension("html");
    let mut file =
        fs::File::create(&outfile).expect("Trouble creating output file");
    file.write_all(mc.pretty_print().as_bytes())
        .expect("Trouble writing output file");
    println!("HTML-Output saved to:\n- {:#?}", outfile);
    _ = self.ed.get_string("Press Enter to continue".italic());
}
```

To:
```rust
CharacterMenu::PrintCharacterSheet => {
    let mut outfile = mc.file_path.clone();
    outfile.set_extension("md");
    let mut file =
        fs::File::create(&outfile).expect("Trouble creating output file");
    file.write_all(mc.pretty_print().as_bytes())
        .expect("Trouble writing output file");
    println!("Markdown-Output saved to:\n- {:#?}", outfile);
    _ = self.ed.get_string("Press Enter to continue".italic());
}
```

### 2. Refactor `pretty_print()` in `src/character.rs`

**Location:** Lines 572-739

Convert all HTML generation to Markdown format:
- Replace `<span class="u">...</span>` with `[...]{.u}`
- Replace `<b>...</b>` with `**...**`
- Replace `<i>...</i>` with `*...*`
- Remove all `<br/>` tags
- Prepend `| ` to each line

### 3. Update `pretty_print()` in Item Type Files

Each of these files needs its `pretty_print()` method updated to generate Markdown:

| File | Type |
|------|------|
| `src/spell.rs` | Spell (lines 208-223) |
| `src/skill.rs` | Skill (lines 135-149) |
| `src/gear.rs` | Gear (lines 129-144) |
| `src/weapon.rs` | Weapon (lines 123-138) |
| `src/traits.rs` | CharacterTrait (lines 45-49) |
| `src/notes.rs` | Note (lines 54-58) |

### HTML to Markdown Conversion Patterns

**Status Section:**
```
<span class="u">Status</span>
<b>Name:</b> ...<br/>
```
→
```
| [Status]{.u}
| **Name:** ...
```

**Attributes with Attunement:**
```
<b>Body:</b> ...<br/>
╰╼(... attunement)<br/>
```
→
```
| **Body:** ...
| ╰╼(... attunement)
```

**Items with modifiers:**
```
<b>Item Name</b> (Type)<br/>
<i>description</i><br/>
- modifier 1<br/>
- modifier 2<br/>
```
→
```
| **Item Name** (Type)
| *description*
| - modifier 1
| - modifier 2
```

**Spells/Skills:**
```
<b>Name:</b> ...<br/>
╰╼(mana/stamina / cooldown)<br/>
```
→
```
| **Name:** ...
| ╰╼(mana/stamina / cooldown)
```

## Files to Modify

1. `src/interactive/mod.rs` - File extension and message
2. `src/character.rs` - Main `pretty_print()` method
3. `src/spell.rs` - Spell output format
4. `src/skill.rs` - Skill output format
5. `src/gear.rs` - Gear output format
6. `src/weapon.rs` - Weapon output format
7. `src/traits.rs` - CharacterTrait output format
8. `src/notes.rs` - Note output format

## Testing

1. Load `Alaric.json`
2. Use "Print Character Sheet" function
3. Verify `Alaric.md` matches expected format
4. Compare with existing manually created `Alaric.md`

## Risks & Considerations

- The pipe `|` prefix is non-standard Markdown - relies on custom encoder
- Removing `<br/>` tags changes output structure significantly
- Attunement display in HTML uses `<i></i><br/>` which becomes just the modifier list in Markdown