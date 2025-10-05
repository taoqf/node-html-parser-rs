// Integration test harness aggregating per-issue tests under tests/issuss
// Add new modules here as more JS issue tests are ported.

mod issues {
	mod issue_100;
	mod issue_106; // ignored memory leak stress (skip parity)
	mod issue_109;
	mod issue_112;
	mod issue_115;
	mod issue_119;
	mod issue_129;
	mod issue_135;
	mod issue_136;
	mod issue_145;
	mod issue_152;
	mod issue_165;
	mod issue_171;
	mod issue_176;
	mod issue_185;
	mod issue_186;
	mod issue_200;
	mod issue_203;
	mod issue_207;
	mod issue_214;
	mod issue_218;
	mod issue_224;
	mod issue_226;
	mod issue_227;
	mod issue_239;
	mod issue_240;
	mod issue_242;
	mod issue_248;
	mod issue_249;
	mod issue_254;
	mod issue_258;
	mod issue_260;
	mod issue_267;
	mod issue_268; // ignored (skipped in JS)
	mod issue_269_270;
	mod issue_274;
	mod issue_277;
	mod issue_279;
	mod issue_280; // large HTML Standard performance sanity
	mod issue_28_59_74;
	mod issue_41;
	mod issue_42;
	mod issue_48;
	mod issue_51;
	mod issue_69; // embed/iframe large snippet
	mod issue_70;
	mod issue_84;
	mod issue_85;
	mod issue_95;
	mod issue_98;
}
