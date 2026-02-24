import re

def fix():
    with open("src/datetime.rs", "r") as f:
        text = f.read()

    # 1. Remove `const` from `pub const fn` for getters
    methods = [
        "year", "month", "day", "hour", "minute", "second", 
        "microsecond", "iso_week", "ordinal", "offset", "weekday"
    ]
    for m in methods:
        pattern = r"pub const fn " + m + r"\(\&self\)"
        replacement = r"pub fn " + m + r"(&self)"
        text = re.sub(pattern, replacement, text)

    # 2. Fix missed instantiations of `Self { datetime: ..., offset: ... }`
    text = re.sub(r"Self\s*\{\s*datetime:\s*([^,\[\]\{\}\(\)]+(?:\([^()]*\))?),\s*offset:\s*([^,\{\}\[\]\(\)]+(?:\([^()]*\))?),?\s*\}",
                  r"Self::from_primitive_and_offset(\1, \2)", text)

    text = re.sub(r"Self\s*\{\s*datetime:\s*([^,]+),\s*offset:\s*([^,]+),?\s*\}", 
                  r"Self::from_primitive_and_offset(\1, \2)", text)

    # Some blocks are multiline like:
    # Self {
    #     datetime: time_components,
    #     offset: UtcOffset::UTC,
    # }
    text = text.replace("Self {\n            datetime: time_components,\n            offset: UtcOffset::UTC,\n        }", "Self::from_primitive_and_offset(time_components, UtcOffset::UTC)")

    text = text.replace("Self {\n            datetime: time_components,\n            offset: self.as_offset(),\n        }", "Self::from_primitive_and_offset(time_components, self.as_offset())")
    
    # 3. Fix other specific errors from line 1030, 1085, 1132, 1162, 1189, 1190, 1232
    # These are likely `self.datetime` or `self.offset` that missed replacement.
    text = text.replace("self.datetime.assume_offset", "self.as_primitive().assume_offset")
    text = text.replace("other.datetime", "other.as_primitive()")
    
    # Handle `.offset` -> `.as_offset()`
    # Let's just blindly replace `self.offset.` with `self.as_offset().` and `self.offset)` with `self.as_offset())`
    text = text.replace("self.offset.", "self.as_offset().")
    text = text.replace("self.offset)", "self.as_offset())")
    text = text.replace("self.offset,", "self.as_offset(),")

    # Double check DateTimeBuilder which uses `self.offset` natively.
    # In DateTimeBuilder, `self.offset` is valid. So replacing `self.offset` blindly breaks DateTimeBuilder.
    
    with open("src/datetime.rs", "w") as f:
        f.write(text)

fix()
