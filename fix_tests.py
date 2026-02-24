import re

def fix():
    with open("tests/test_datetime.rs", "r") as f:
        text = f.read()

    # We need to replace `DateTime { datetime, offset }` with the primitive initialization.
    text = text.replace(
        "let dt = DateTime { datetime, offset };",
        "let dt = DateTime { unix_seconds: datetime.assume_offset(offset).unix_timestamp(), nanoseconds: datetime.nanosecond(), utc_offset_minutes: (offset.whole_seconds() / 60) as i16 };"
    )

    text = text.replace(
        "datetime: datetime2,\n                offset: offset2,",
        "unix_seconds: datetime2.assume_offset(offset2).unix_timestamp(), nanoseconds: datetime2.nanosecond(), utc_offset_minutes: (offset2.whole_seconds() / 60) as i16,"
    )

    # dt.offset -> dt.offset()
    text = text.replace("dt.offset,", "dt.offset(),")
    text = text.replace("dt.offset)", "dt.offset())")
    text = text.replace("dt.offset == ", "dt.offset() == ")
    text = text.replace("dt.offset!=", "dt.offset()!=")
    
    with open("tests/test_datetime.rs", "w") as f:
        f.write(text)

fix()
