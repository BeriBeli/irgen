import { SpreadsheetFile, Workbook } from "@oai/artifact-tool";

const workbook = Workbook.create();
const version = workbook.worksheets.add("version");
const addressMap = workbook.worksheets.add("address_map");
const regs = workbook.worksheets.add("regs");

version.getRange("A1:D2").values = [
  ["VENDOR", "LIBRARY", "NAME", "VERSION"],
  ["example.com", "IP", "example_simple", "1.0"],
];

addressMap.getRange("A1:C2").values = [
  ["BLOCK", "OFFSET", "RANGE"],
  ["regs", "0x0", "0x20"],
];

regs.getRange("A1:H3").values = [
  ["ADDR", "REG", "FIELD", "BIT", "WIDTH", "ATTRIBUTE", "DEFAULT", "DESCRIPTION"],
  ["0x0", "status", "ready", "[31:0]", "32", "RO", "0", "ready flag"],
  ["0x4", "control", "enable", "[31:0]", "32", "RW", "0", "enable control"],
];

for (const sheet of [version, addressMap, regs]) {
  sheet.getUsedRange().format.autofitColumns();
}

const output = await SpreadsheetFile.exportXlsx(workbook);
await output.save("example_simple.xlsx");

const inspect = await workbook.inspect({
  kind: "table",
  range: "regs!A1:H3",
  include: "values",
  tableMaxRows: 5,
  tableMaxCols: 8,
});
console.log(inspect.ndjson);
