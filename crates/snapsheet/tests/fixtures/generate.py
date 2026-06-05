#!/usr/bin/env python3
"""Generate minimal invalid XLSX fixtures for integration tests."""

from pathlib import Path
from xml.sax.saxutils import escape
from zipfile import ZIP_DEFLATED, ZipFile, ZipInfo

FIXTURE_DIR = Path(__file__).parent
HEADERS = ["ADDR", "REG", "REG_DESC", "FIELD", "BIT", "ATTR", "RESET", "FIELD_DESC"]
FIXED_TIMESTAMP = (1980, 1, 1, 0, 0, 0)

CONTENT_TYPES = """<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
  <Override PartName="/xl/worksheets/sheet2.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
  <Override PartName="/xl/worksheets/sheet3.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
</Types>
"""

ROOT_RELS = """<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/>
</Relationships>
"""

WORKBOOK = """<?xml version="1.0" encoding="UTF-8"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
          xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <sheets>
    <sheet name="version" sheetId="1" r:id="rId1"/>
    <sheet name="address_map" sheetId="2" r:id="rId2"/>
    <sheet name="regs" sheetId="3" r:id="rId3"/>
  </sheets>
</workbook>
"""

WORKBOOK_RELS = """<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet2.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet3.xml"/>
</Relationships>
"""

VERSION_ROWS = [
    ["VENDOR", "LIBRARY", "NAME", "VERSION"],
    ["example.com", "IP", "invalid-fixture", "1.0"],
]

ADDRESS_MAP_ROWS = [
    ["BLOCK", "OFFSET", "RANGE"],
    ["regs", "0", "4"],
]

FIXTURES = {
    "conflicting_registers.xlsx": [
        HEADERS,
        ["0", "reg", "", "value", "[31:0]", "RW", "0", ""],
        ["4", "reg", "", "value", "[31:0]", "RW", "0", ""],
    ],
    "duplicate_fields.xlsx": [
        HEADERS,
        ["0", "reg", "", "value", "[31:16]", "RW", "0", ""],
        ["", "", "", "value", "[15:0]", "RW", "0", ""],
    ],
    "overlapping_fields.xlsx": [
        HEADERS,
        ["0", "reg", "", "high", "[31:8]", "RW", "0", ""],
        ["", "", "", "low", "[15:0]", "RW", "0", ""],
    ],
    "invalid_attribute.xlsx": [
        HEADERS,
        ["0", "reg", "", "value", "[31:0]", "BAD", "0", ""],
    ],
    "malformed_range.xlsx": [
        HEADERS,
        ["0", "reg{n}, n=range(0, nope)", "", "value", "[31:0]", "RW", "0", ""],
    ],
    "out_of_range_register.xlsx": [
        HEADERS,
        ["4", "reg", "", "value", "[31:0]", "RW", "0", ""],
    ],
}


def column_name(index: int) -> str:
    name = ""
    while index:
        index, remainder = divmod(index - 1, 26)
        name = chr(ord("A") + remainder) + name
    return name


def worksheet(rows: list[list[str]]) -> str:
    rendered_rows = []
    for row_index, row in enumerate(rows, start=1):
        cells = []
        for column_index, value in enumerate(row, start=1):
            if value == "":
                continue
            reference = f"{column_name(column_index)}{row_index}"
            cells.append(
                f'<c r="{reference}" t="inlineStr"><is><t>{escape(value)}</t></is></c>'
            )
        rendered_rows.append(f'<row r="{row_index}">{"".join(cells)}</row>')
    return (
        '<?xml version="1.0" encoding="UTF-8"?>'
        '<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">'
        f'<sheetData>{"".join(rendered_rows)}</sheetData>'
        "</worksheet>"
    )


def write_file(archive: ZipFile, path: str, content: str) -> None:
    info = ZipInfo(path, FIXED_TIMESTAMP)
    info.compress_type = ZIP_DEFLATED
    archive.writestr(info, content.encode("utf-8"))


def write_fixture(path: Path, register_rows: list[list[str]]) -> None:
    with ZipFile(path, "w") as archive:
        write_file(archive, "[Content_Types].xml", CONTENT_TYPES)
        write_file(archive, "_rels/.rels", ROOT_RELS)
        write_file(archive, "xl/workbook.xml", WORKBOOK)
        write_file(archive, "xl/_rels/workbook.xml.rels", WORKBOOK_RELS)
        write_file(archive, "xl/worksheets/sheet1.xml", worksheet(VERSION_ROWS))
        write_file(archive, "xl/worksheets/sheet2.xml", worksheet(ADDRESS_MAP_ROWS))
        write_file(archive, "xl/worksheets/sheet3.xml", worksheet(register_rows))


for filename, rows in FIXTURES.items():
    write_fixture(FIXTURE_DIR / filename, rows)
