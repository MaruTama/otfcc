// Regression test for issue #1: a large `gsub_alternate` lookup whose single
// serialized subtable exceeds the 16-bit offset limit (64KB) used to be
// silently corrupted (garbage Coverage format, lookup dropped on re-dump).
// The build path now splits oversized Multiple/Alternate subtables so every
// entry survives a build -> dump round trip.
//
// Usage: node gsub-alternate-large-test.js <bindir> <baseFont.ttf>
//   <bindir>       directory containing otfccbuild / otfccdump
//   <baseFont.ttf> a payload font providing enough glyphs to cover from-glyphs

var fs = require("fs");
var path = require("path");
var os = require("os");
var cp = require("child_process");

function check(cond, desc, extra) {
	if (cond) {
		process.stderr.write("\x1b[32;1m[PASS]\x1b[39;49m " + desc + "\n");
	} else {
		process.stderr.write("\x1b[31;1m[FAIL]\x1b[39;49m " + desc + "\n");
		if (extra !== undefined) process.stderr.write(String(extra) + "\n");
		process.exit(1);
	}
}

var binDir = process.argv[2];
var baseFont = process.argv[3];
var otfccdump = path.join(binDir, "otfccdump");
var otfccbuild = path.join(binDir, "otfccbuild");

var tmp = fs.mkdtempSync(path.join(os.tmpdir(), "gsub-alt-"));
var baseJson = path.join(tmp, "base.json");
var bigTtf = path.join(tmp, "big.ttf");
var redumpJson = path.join(tmp, "redump.json");

function run(cmd, args) {
	var r = cp.spawnSync(cmd, args, { stdio: ["ignore", "inherit", "inherit"] });
	if (r.status !== 0) {
		process.stderr.write("\x1b[31;1m[FAIL]\x1b[39;49m command failed: " + cmd + " " + args.join(" ") + "\n");
		process.exit(1);
	}
}

// 1. Dump the base font to JSON.
run(otfccdump, [baseFont, "-o", baseJson, "--pretty"]);
var font = JSON.parse(fs.readFileSync(baseJson, "utf8"));
var glyphs = Object.keys(font.glyf);
check(glyphs.length > 100, "base font has enough glyphs (" + glyphs.length + ")");

// 2. Inject a large gsub_alternate lookup. Each from-glyph maps to M alternates;
//    with all glyphs covered this pushes the single subtable well past 64KB.
var M = 12;
var sub = {};
for (var i = 0; i < glyphs.length; i++) {
	var alts = [];
	for (var k = 0; k < M; k++) alts.push(glyphs[(i + 1 + k) % glyphs.length]);
	sub[glyphs[i]] = alts;
}
var estBytes = 6 + glyphs.length * 2 + glyphs.length * (2 + M * 2) + 4 + glyphs.length * 2;
check(estBytes > 0xFFFF, "synthetic subtable exceeds 64KB (est " + estBytes + " bytes)");

if (!font.GSUB) font.GSUB = { languages: {}, features: {}, lookups: {}, lookupOrder: [] };
font.GSUB.lookups["lookup_bigalt_0"] = { type: "gsub_alternate", flags: {}, subtables: [sub] };
font.GSUB.lookupOrder.push("lookup_bigalt_0");
font.GSUB.features["aalt_bigtest"] = ["lookup_bigalt_0"];
var langs = Object.keys(font.GSUB.languages);
if (langs.length === 0) {
	font.GSUB.languages["DFLT_DFLT"] = { features: ["aalt_bigtest"] };
} else {
	for (var l = 0; l < langs.length; l++) {
		var lang = font.GSUB.languages[langs[l]];
		if (lang && lang.features) lang.features.push("aalt_bigtest");
	}
}
fs.writeFileSync(baseJson, JSON.stringify(font));

// 3. Build to TTF and dump back.
run(otfccbuild, [baseJson, "-o", bigTtf, "--keep-average-char-width", "--keep-modified-time"]);
run(otfccdump, [bigTtf, "-o", redumpJson, "--pretty"]);
var out = JSON.parse(fs.readFileSync(redumpJson, "utf8"));

// 4. The lookup is renamed after its feature (aalt) on dump. Find the surviving
//    gsub_alternate lookup(s) referenced by the aalt feature and merge entries.
check(out.GSUB, "output font still has a GSUB table");
var merged = {};
var subtableCount = 0;
Object.keys(out.GSUB.lookups).forEach(function (name) {
	var lk = out.GSUB.lookups[name];
	if (lk.type !== "gsub_alternate") return;
	lk.subtables.forEach(function (st) {
		subtableCount++;
		Object.keys(st).forEach(function (g) { merged[g] = st[g]; });
	});
});

check(subtableCount >= 2, "oversized subtable was split into multiple subtables (" + subtableCount + ")");

var origKeys = Object.keys(sub);
var got = Object.keys(merged).length;
check(got === origKeys.length, "all " + origKeys.length + " from-glyphs preserved (got " + got + ")");

var mismatches = 0;
for (var m = 0; m < origKeys.length; m++) {
	var key = origKeys[m];
	if (JSON.stringify(merged[key]) !== JSON.stringify(sub[key])) mismatches++;
}
check(mismatches === 0, "every alternate set is byte-identical after round trip", mismatches + " mismatches");

fs.rmSync(tmp, { recursive: true, force: true });
process.stderr.write("\x1b[32;1m[PASS]\x1b[39;49m gsub_alternate large-lookup regression test\n");
