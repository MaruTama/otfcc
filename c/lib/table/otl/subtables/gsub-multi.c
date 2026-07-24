#include "gsub-multi.h"

static void deleteGsubMultiEntry(otl_GsubMultiEntry *entry) {
	Handle.dispose(&entry->from);
	DELETE(Coverage.free, entry->to);
}

static caryll_ElementInterface(otl_GsubMultiEntry) gsm_typeinfo = {
    .init = NULL, .copy = NULL, .dispose = deleteGsubMultiEntry};

caryll_standardVectorImpl(subtable_gsub_multi, otl_GsubMultiEntry, gsm_typeinfo,
                          iSubtable_gsub_multi);

otl_Subtable *otl_read_gsub_multi(font_file_pointer data, uint32_t tableLength, uint32_t offset,
                                  const glyphid_t maxGlyphs, const otfcc_Options *options) {
	subtable_gsub_multi *subtable = iSubtable_gsub_multi.create();
	otl_Coverage *from = NULL;
	checkLength(offset + 6);

	from = Coverage.read(data, tableLength, offset + read_16u(data + offset + 2));
	glyphid_t seqCount = read_16u(data + offset + 4);
	if (seqCount != from->numGlyphs) goto FAIL;
	checkLength(offset + 6 + seqCount * 2);

	for (glyphid_t j = 0; j < seqCount; j++) {
		uint32_t seqOffset = offset + read_16u(data + offset + 6 + j * 2);
		otl_Coverage *cov = Coverage.create();
		glyphid_t n = read_16u(data + seqOffset);
		for (glyphid_t k = 0; k < n; k++) {
			Coverage.push(cov, Handle.fromIndex(read_16u(data + seqOffset + 2 + k * 2)));
		}
		iSubtable_gsub_multi.push(subtable, ((otl_GsubMultiEntry){
		                                        .from = Handle.dup(from->glyphs[j]), .to = cov,
		                                    }));
	}
	Coverage.free(from);
	return (otl_Subtable *)subtable;

FAIL:
	if (from) Coverage.free(from);
	iSubtable_gsub_multi.free(subtable);
	return NULL;
}

json_value *otl_gsub_dump_multi(const otl_Subtable *_subtable) {
	const subtable_gsub_multi *subtable = &(_subtable->gsub_multi);
	json_value *st = json_object_new(subtable->length);
	for (glyphid_t j = 0; j < subtable->length; j++) {
		json_object_push(st, subtable->items[j].from.name, Coverage.dump(subtable->items[j].to));
	}
	return st;
}

otl_Subtable *otl_gsub_parse_multi(const json_value *_subtable, const otfcc_Options *options) {
	subtable_gsub_multi *st = iSubtable_gsub_multi.create();

	for (glyphid_t k = 0; k < _subtable->u.object.length; k++) {
		json_value *_to = _subtable->u.object.values[k].value;
		if (!_to || _to->type != json_array) continue;
		iSubtable_gsub_multi.push(
		    st, ((otl_GsubMultiEntry){
		            .from = Handle.fromName(sdsnewlen(_subtable->u.object.values[k].name,
		                                              _subtable->u.object.values[k].name_length)),
		            .to = Coverage.parse(_to),
		        }));
	}

	return (otl_Subtable *)st;
}

// Builds a single Multiple/Alternate substitution subtable covering the
// entries in the half-open range [start, end).
static caryll_Buffer *buildGsubMultiSubtableRange(const subtable_gsub_multi *subtable,
                                                  glyphid_t start, glyphid_t end) {
	otl_Coverage *cov = Coverage.create();
	for (glyphid_t j = start; j < end; j++) {
		Coverage.push(cov, Handle.dup(subtable->items[j].from));
	}

	bk_Block *root = bk_new_Block(b16, 1,                                          // format
	                              p16, bk_newBlockFromBuffer(Coverage.build(cov)), // coverage
	                              b16, end - start,                                // quantity
	                              bkover);
	for (glyphid_t j = start; j < end; j++) {
		bk_Block *b = bk_new_Block(b16, subtable->items[j].to->numGlyphs, bkover);
		for (glyphid_t k = 0; k < subtable->items[j].to->numGlyphs; k++) {
			bk_push(b, b16, subtable->items[j].to->glyphs[k].index, bkover);
		}
		bk_push(root, p16, b, bkover);
	}
	Coverage.free(cov);
	return bk_build_Block(root);
}

// A Multiple/Alternate subtable uses 16-bit internal offsets (Coverage offset
// and one offset per Sequence/AlternateSet). When a single subtable grows past
// 64KB these offsets silently overflow, corrupting the whole table (see #1).
// We therefore split large subtables into several, each staying below the limit.
// Keep a safety margin under 0xFFFF because the per-entry size below is an
// upper-bound estimate (Coverage.build may pick the smaller of format 1 / 2).
#define GSUB_MULTI_SUBTABLE_SIZE_LIMIT 0xFF00

caryll_Buffer **otfcc_build_gsub_multi_subtable_split(const otl_Subtable *_subtable,
                                                      otl_BuildHeuristics heuristics,
                                                      tableid_t *count) {
	const subtable_gsub_multi *subtable = &(_subtable->gsub_multi);

	caryll_Buffer **parts = NULL;
	tableid_t nParts = 0;
	glyphid_t start = 0;
	while (start < subtable->length) {
		// Fixed subtable overhead: format(2) + coverageOffset(2) + quantity(2).
		// Coverage upper bound (format 1): 4 + 2 * n. Per Sequence offset: 2.
		size_t size = 6 + 4;
		glyphid_t end = start;
		while (end < subtable->length) {
			// cost of adding entry `end`: coverage glyph (2) + sequence offset (2)
			//   + AlternateSet/Sequence table: count(2) + numGlyphs * 2
			size_t entrySize = 2 + 2 + 2 + (size_t)subtable->items[end].to->numGlyphs * 2;
			// Always keep at least one entry per subtable to guarantee progress.
			if (end > start && size + entrySize > GSUB_MULTI_SUBTABLE_SIZE_LIMIT) break;
			size += entrySize;
			end++;
		}
		RESIZE(parts, nParts + 1);
		parts[nParts] = buildGsubMultiSubtableRange(subtable, start, end);
		nParts++;
		start = end;
	}
	if (!nParts) {
		// Empty subtable: still emit one (empty) buffer to preserve behavior.
		RESIZE(parts, 1);
		parts[0] = buildGsubMultiSubtableRange(subtable, 0, 0);
		nParts = 1;
	}
	*count = nParts;
	return parts;
}

caryll_Buffer *otfcc_build_gsub_multi_subtable(const otl_Subtable *_subtable, otl_BuildHeuristics heuristics) {
	const subtable_gsub_multi *subtable = &(_subtable->gsub_multi);
	return buildGsubMultiSubtableRange(subtable, 0, subtable->length);
}
