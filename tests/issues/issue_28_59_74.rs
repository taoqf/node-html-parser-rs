use node_html_parser::parse;

#[test]
fn issue_28_query_dl_dt_direct_child() {
	let html = "<dl>\n  <dt>A</dt>\n  <dd>B</dd>\n  <dt>C</dt>\n  <dd>D</dt>\n</dl>\n";
	let root = parse(html);
	let els = root.query_selector_all("dl > dt");
	assert_eq!(els.len(), 2);
	assert_eq!(els[0].inner_html(), "A");
	assert_eq!(els[1].inner_html(), "C");
}

#[test]
fn issue_28_query_dl_dt_and_dd_group() {
	let html = "<dl>\n  <dt>A</dt>\n  <dd>B</dd>\n  <dt>C</dt>\n  <dd>D</dd>\n</dl>\n";
	let root = parse(html);
	let els = root.query_selector_all("dl dt, dl dd");
	assert_eq!(els.len(), 4);
	assert_eq!(els[0].inner_html(), "A");
	assert_eq!(els[1].inner_html(), "B");
	assert_eq!(els[2].inner_html(), "C");
	assert_eq!(els[3].inner_html(), "D");
}

#[test]
fn issue_28_class_chain() {
	let html = "<div class=\"a b\"></div>";
	let root = parse(html);
	let el = root.query_selector(".a.b").unwrap();
	assert_eq!(el.name().to_ascii_uppercase(), "DIV");
}

#[test]
fn issue_28_ul_with_item_attr() {
	let html = "<ul item=\"111\" id=\"list\"><li>Hello World</li></ul>";
	let root = parse(html);
	let ul = root.query_selector("ul[item]").unwrap();
	assert_eq!(ul.name().to_ascii_uppercase(), "UL");
	let li = root.query_selector("ul#list").unwrap();
	assert_eq!(li.text(), "Hello World");
}

#[test]
fn issue_59_tr_td_nth_child_2() {
	let html = "<tr>\n  <td>\n    <div>\n      <span>\n        a\n\t\t\t</span>\n      <p>\n        b\n\t\t\t</p>\n      <p>\n        c\n\t\t\t</p>\n\t\t</div>\n  <td>ddd</td>\n  <td>\n    <span>\n      eee\n\t\t</span>\n\t</td>\n</tr>\n";
	let root = parse(html);
	let el = root.query_selector("tr td:nth-child(2)").unwrap();
	assert_eq!(el.inner_html(), "ddd");
}

#[test]
fn issue_74_td_nth_child_6_a_href() {
	let html = "<table>\n<tbody>\n   <tr class=\"odd\">\n      <td>13/10/2020</td>\n      <td>\n         Cell2\n      </td>\n      <td>\n         <a href=\"/mjrcs-32432\">Cell3</a>\n      </td>\n      <td>\n         <a target=\"_blank\" href=\"/5z9LX1.pdf\" ><img alt=\"PDF File\" src=\"/mjrcs-resa/images/v2/icone_pdf.gif\"></a>\n      </td>\n      <td>\n         <a target=\"winzip\" href=\"/33WzKc.zip\"><img alt=\"ZIP File\" src=\"/mjrcs-resa/images/v2/icone_pdf_archive.gif\" height=\"16\" ></a>\n      </td>\n      <td>\n         <a target=\"_blank\" href=\"/3QhZGq.xml\"><img alt=\"XML file\"  src=\"/mjrcs-resa/images/v2/icone_xml.gif\" height=\"16\" ></a>\n      </td>\n      <td>\n      </td>\n   </tr>\n</tbody>\n</table>\n";
	let root = parse(html);
	let el = root
		.query_selector("table tbody tr td:nth-child(6) a")
		.unwrap();
	let href = el.get_attr("href").unwrap();
	assert_eq!(href, "/3QhZGq.xml");
}
