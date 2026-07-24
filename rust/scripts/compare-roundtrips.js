// Runs tests/ttf-roundtrip-test.js over every payload rust/scripts/test.sh
// produced under build/rust-test/, exiting non-zero if any fail.
var cp = require("child_process");
var fs = require("fs");
var path = require("path");

var TTF_PAYLOADS = "NotoNastaliqUrdu-Regular iosevka-r BungeeColor-Regular_colr_Windows Reinebow-SVGinOT vtt Molengo-Regular".split(" ");
var CFF_PAYLOADS = "KRName-Regular".split(" ");
var CFF_FJ_PAYLOADS = "WorkSans-Regular kltf-bugfont1".split(" ");

var BUILD = path.join(__dirname, "..", "..", "build", "rust-test");
var TESTJS = path.join(__dirname, "..", "..", "tests", "ttf-roundtrip-test.js");

var names = TTF_PAYLOADS.concat(CFF_PAYLOADS).map(function (n) { return { label: n, base: n }; })
	.concat(CFF_FJ_PAYLOADS.map(function (n) { return { label: n + " (fj)", base: "fj-" + n }; }));

// Optional: only present if make-test-variable-font.py + run-cycles.sh
// generated it (needs fontTools).
if (fs.existsSync(path.join(BUILD, "gvar-test.5.json"))) {
	names.push({ label: "gvar-test", base: "gvar-test" });
}

var failed = 0;
names.forEach(function (n) {
	var five = path.join(BUILD, n.base + ".5.json");
	var three = path.join(BUILD, n.base + ".3.json");
	var r = cp.spawnSync("node", [TESTJS, five, three], { stdio: "inherit" });
	if (r.status !== 0) failed++;
});

if (failed) {
	process.stderr.write("\x1b[31;1m" + failed + " payload(s) failed the round-trip test\x1b[39;49m\n");
	process.exit(1);
} else {
	process.stderr.write("\x1b[32;1mAll " + names.length + " payloads passed\x1b[39;49m\n");
}
