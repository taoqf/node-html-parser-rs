// issue 268: skipped in JS suite (malformed HTML attribute edge cases)
// Replicated here but ignored to mirror original skip.
use node_html_parser::parse;

#[test]
#[ignore]
fn issue_268_malformed_html_skipped() {
	let html = r##"
<table id="mytable">
<tr class="myrow">
  <td>1</td>
  <td><a href="#" 2'>x</a></td>
  <td>2</td>
</tr>
<tr class="myrow">
  <td>3</td>
  <td><a href="#" 2'>x</a></td>
  <td>4</td>
</tr>
</table>"##;
	let root = parse(html);
	let trs: Vec<_> = root.query_selector_all("#mytable tr.myrow");
	assert_eq!(trs.len(), 2);
}
