import re

def refactor():
    with open("src/datetime.rs", "r") as f:
        text = f.read()

    # 1. Replace struct definition
    old_struct = r"""pub struct DateTime \{
    /// The date and time in UTC \(when offset = `UtcOffset::UTC`\) or a
    /// user-chosen offset if `offset != UtcOffset::UTC`.
    pub datetime: PrimitiveDateTime,
    /// The timezone offset from UTC.
    pub offset: UtcOffset,
\}"""

    new_struct = """pub struct DateTime {
    /// The exact Unix timestamp in seconds.
    pub unix_seconds: i64,
    /// The fractional nanoseconds (0-999,999,999).
    pub nanoseconds: u32,
    /// The timezone offset from UTC in minutes.
    pub utc_offset_minutes: i16,
}

impl DateTime {
    #[inline]
    fn as_offset(&self) -> UtcOffset {
        UtcOffset::from_whole_seconds(self.utc_offset_minutes as i32 * 60).unwrap_or(UtcOffset::UTC)
    }

    #[inline]
    fn as_primitive(&self) -> PrimitiveDateTime {
        let dt = time::OffsetDateTime::from_unix_timestamp_nanos(
            (self.unix_seconds as i128 * 1_000_000_000) + self.nanoseconds as i128
        ).unwrap_or_else(|_| time::OffsetDateTime::now_utc());
        let dt_local = dt.to_offset(self.as_offset());
        PrimitiveDateTime::new(dt_local.date(), dt_local.time())
    }

    #[inline]
    fn from_primitive_and_offset(dt: PrimitiveDateTime, offset: UtcOffset) -> Self {
        let odt = dt.assume_offset(offset);
        Self {
            unix_seconds: odt.unix_timestamp(),
            nanoseconds: odt.nanosecond(),
            utc_offset_minutes: (offset.whole_seconds() / 60) as i16,
        }
    }
}"""
    text = re.sub(old_struct, new_struct, text, count=1)

    # 2. Replace structural instantiations
    # e.g., Self { datetime: ..., offset: ... } -> Self::from_primitive_and_offset(..., ...)
    # Need to handle multiline.
    pattern = r"Self\s*\{\s*datetime:\s*([^,]+(?:,\s*[^,]+)*),\s*offset:\s*([^,}]+),?\s*\}"
    
    # We will do this manually for the 7 instances:
    text = text.replace("Self {\n            datetime: PrimitiveDateTime::new(now.date(), now.time()),\n            offset: UtcOffset::UTC,\n        }", "Self::from_primitive_and_offset(PrimitiveDateTime::new(now.date(), now.time()), UtcOffset::UTC)")
    
    text = text.replace("Self {\n            datetime: PrimitiveDateTime::new(\n                now_local.date(),\n                now_local.time(),\n            ),\n            offset: *offset,\n        }", "Self::from_primitive_and_offset(PrimitiveDateTime::new(now_local.date(), now_local.time()), *offset)")
    
    text = text.replace("Self {\n            datetime: PrimitiveDateTime::new(now.date(), now.time()),\n            offset: UtcOffset::UTC,\n        }", "Self::from_primitive_and_offset(PrimitiveDateTime::new(now.date(), now.time()), UtcOffset::UTC)")

    text = text.replace("Self {\n            datetime: PrimitiveDateTime::new(\n                now_local.date(),\n                now_local.time(),\n            ),\n            offset,\n        }", "Self::from_primitive_and_offset(PrimitiveDateTime::new(now_local.date(), now_local.time()), offset)")

    text = text.replace("Self {\n            datetime: new_dt,\n            offset: self.offset,\n        }", "Self::from_primitive_and_offset(new_dt, self.as_offset())")
    text = text.replace("Self {\n            datetime: PrimitiveDateTime::new(\n                self.datetime.date(),\n                new_time,\n            ),\n            offset: self.offset,\n        }", "Self::from_primitive_and_offset(PrimitiveDateTime::new(self.as_primitive().date(), new_time), self.as_offset())")
    
    text = text.replace("Self {\n            datetime: PrimitiveDateTime::new(\n                self.datetime.date(),\n                new_time,\n            ),\n            offset: self.offset,\n        }", "Self::from_primitive_and_offset(PrimitiveDateTime::new(self.as_primitive().date(), new_time), self.as_offset())")

    text = text.replace("Self {\n                datetime: PrimitiveDateTime::new(\n                    self.datetime.date(),\n                    new_time,\n                ),\n                offset: self.offset,\n            }", "Self::from_primitive_and_offset(PrimitiveDateTime::new(self.as_primitive().date(), new_time), self.as_offset())")

    text = text.replace("Self {\n            datetime: PrimitiveDateTime::new(new_date, self.datetime.time()),\n            offset: self.offset,\n        }", "Self::from_primitive_and_offset(PrimitiveDateTime::new(new_date, self.as_primitive().time()), self.as_offset())")

    text = text.replace("Self {\n                datetime: PrimitiveDateTime::new(new_date, self.datetime.time()),\n                offset: self.offset,\n            }", "Self::from_primitive_and_offset(PrimitiveDateTime::new(new_date, self.as_primitive().time()), self.as_offset())")
    
    text = text.replace("Self {\n                datetime: PrimitiveDateTime::new(\n                    self.datetime.date(),\n                    self.datetime.time(),\n                ),\n                offset: self.offset,\n            }", "Self::from_primitive_and_offset(PrimitiveDateTime::new(self.as_primitive().date(), self.as_primitive().time()), self.as_offset())")
    
    # 3. Replace all other `.datetime`
    text = text.replace("self.datetime", "self.as_primitive()")

    # 4. Replace `.offset` (except when it's already `.offset()`)
    # Use regex to find `self.offset` that is NOT followed by `)` or `(`
    text = re.sub(r"self\.offset(?![\(\)])", "self.as_offset()", text)
    
    # Some other places might have destructuring or direct assignments, we will clean them up.
    # Handle the `.cmp(&other.datetime)` -> `.cmp(&other.as_primitive())`
    text = text.replace("other.datetime", "other.as_primitive()")
    
    # Fix the `offset: offset` in `from_components`
    text = text.replace("Self {\n            datetime: PrimitiveDateTime::new(date, time),\n            offset,\n        }", "Self::from_primitive_and_offset(PrimitiveDateTime::new(date, time), offset)")
    
    text = text.replace("Self {\n            datetime: PrimitiveDateTime::new(\n                now_utc.date(),\n                now_utc.time(),\n            ),\n            offset: self.offset,\n        }", "Self::from_primitive_and_offset(PrimitiveDateTime::new(now_utc.date(), now_utc.time()), self.as_offset())")

    # The manual replaces above might miss a few due to slight whitespace. 
    # Let's do a fallback regex for block struct initialization:
    text = re.sub(r"Self\s*\{\s*datetime:\s*([^,]+(?:\n[^\n]+){0,5}),\s*offset:\s*([^\}]+)\s*\}", r"Self::from_primitive_and_offset(\1, \2)", text)

    with open("src/datetime.rs", "w") as f:
        f.write(text)

        
refactor()
