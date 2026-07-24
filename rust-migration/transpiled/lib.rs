#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(raw_ref_op)]

pub mod src {
    pub mod dep {
        pub mod r#extern {
            pub mod emyg_dtoa {
                pub mod emyg_dtoa;
            } // mod emyg_dtoa
            pub mod json;
            pub mod json_builder;
            pub mod sds;
        } // mod r#extern
    } // mod dep
    pub mod lib {
        pub mod bk {
            pub mod bkblock;
            pub mod bkgraph;
        } // mod bk
        pub mod consolidate {
            pub mod consolidate;
            pub mod otl {
                pub mod GDEF;
                pub mod chaining;
                pub mod common;
                pub mod gpos_cursive;
                pub mod gpos_pair;
                pub mod gpos_single;
                pub mod gsub_ligature;
                pub mod gsub_multi;
                pub mod gsub_reverse;
                pub mod gsub_single;
                pub mod mark;
            } // mod otl
        } // mod consolidate
        pub mod font {
            pub mod caryll_font;
            pub mod caryll_sfnt;
            pub mod caryll_sfnt_builder;
        } // mod font
        pub mod json_reader {
            pub mod json_reader;
        } // mod json_reader
        pub mod json_writer {
            pub mod json_writer;
        } // mod json_writer
        pub mod libcff {
            pub mod cff_charset;
            pub mod cff_codecs;
            pub mod cff_dict;
            pub mod cff_fdselect;
            pub mod cff_index;
            pub mod cff_opmean;
            pub mod cff_parser;
            pub mod cff_string;
            pub mod cff_value;
            pub mod cff_writer;
            pub mod charstring_il;
            pub mod subr;
        } // mod libcff
        pub mod logger {
            pub mod logger;
        } // mod logger
        pub mod otf_reader {
            pub mod otf_reader;
            pub mod unconsolidate;
        } // mod otf_reader
        pub mod otf_writer {
            pub mod otf_writer;
            pub mod stat;
        } // mod otf_writer
        pub mod support {
            pub mod aglfn {
                pub mod aglfn;
            } // mod aglfn
            pub mod alloc;
            pub mod binio;
            pub mod cvec;
            pub mod base64 {
                pub mod base64;
            } // mod base64
            pub mod buffer {
                pub mod buffer;
            } // mod buffer
            pub mod glyph_order;
            pub mod handle;
            pub mod json {
                pub mod json_ident;
            } // mod json
            pub mod options;
            pub mod primitives;
            pub mod sha1 {
                pub mod sha1;
            } // mod sha1
            pub mod ttinstr {
                pub mod ttinstr;
            } // mod ttinstr
            pub mod unicodeconv {
                pub mod unicodeconv;
            } // mod unicodeconv
        } // mod support
        pub mod table {
            pub mod BASE;
            pub mod CFF;
            pub mod COLR;
            pub mod CPAL;
            pub mod GDEF;
            pub mod LTSH;
            pub mod OS_2;
            pub mod SVG;
            pub mod TSI5;
            pub mod VORG;
            pub mod _TSI;
            pub mod cmap;
            pub mod cvt;
            pub mod fpgm_prep;
            pub mod fvar;
            pub mod gasp;
            pub mod glyf {
                pub mod build;
                pub mod glyf;
                pub mod read;
            } // mod glyf
            pub mod hdmx;
            pub mod head;
            pub mod hhea;
            pub mod hmtx;
            pub mod maxp;
            pub mod meta {
                pub mod build;
                pub mod dump;
                pub mod parse;
                pub mod read;
                pub mod r#type;
            } // mod meta
            pub mod name;
            pub mod otl {
                pub mod build;
                pub mod classdef;
                pub mod constants;
                pub mod coverage;
                pub mod dump;
                pub mod otl;
                pub mod parse;
                pub mod read;
                pub mod subtables {
                    pub mod chaining {
                        pub mod build;
                        pub mod classifier;
                        pub mod common;
                        pub mod dump;
                        pub mod parse;
                        pub mod read;
                    } // mod chaining
                    pub mod extend;
                    pub mod gpos_common;
                    pub mod gpos_cursive;
                    pub mod gpos_mark_to_ligature;
                    pub mod gpos_mark_to_single;
                    pub mod gpos_pair;
                    pub mod gpos_single;
                    pub mod gsub_ligature;
                    pub mod gsub_multi;
                    pub mod gsub_reverse;
                    pub mod gsub_single;
                } // mod subtables
            } // mod otl
            pub mod post;
            pub mod vdmx {
                pub mod funcs;
                pub mod r#type;
            } // mod vdmx
            pub mod vhea;
            pub mod vmtx;
        } // mod table
        pub mod vf {
            pub mod axis;
            pub mod region;
            pub mod vq;
        } // mod vf
    } // mod lib
    pub mod src {
        pub mod otfccdll;
        pub mod stopwatch;
    } // mod src
} // mod src
