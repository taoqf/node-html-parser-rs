use node_html_parser::{parse_with_options, Options};

fn first_el(html: &str) -> Box<node_html_parser::HTMLElement> {
	Box::new(
		parse_with_options(html, &Options::default())
			.first_element_child()
			.unwrap()
			.clone(),
	)
}

#[test]
fn attr_transition_duration_literal() {
	let el = first_el("<div x-transition.duration.500ms></div>");
	let mut e = (*el).clone();
	assert_eq!(
		e.get_attribute("x-transition.duration.500ms").as_deref(),
		Some("")
	);
	assert_eq!(el.to_string(), "<div x-transition.duration.500ms></div>");
}

#[test]
fn attr_transition_enter_leave() {
	let el =
		first_el("<div x-transition:enter.duration.500ms x-transition:leave.duration.400ms></div>");
	let mut e = (*el).clone();
	assert_eq!(
		e.get_attribute("x-transition:enter.duration.500ms")
			.as_deref(),
		Some("")
	);
	assert_eq!(
		e.get_attribute("x-transition:leave.duration.400ms")
			.as_deref(),
		Some("")
	);
	assert_eq!(
		el.to_string(),
		"<div x-transition:enter.duration.500ms x-transition:leave.duration.400ms></div>"
	);
}

#[test]
fn attr_click_with_value() {
	let el = first_el("<button @click=\"open = ! open\">Toggle</button>");
	let mut e = (*el).clone();
	assert_eq!(e.get_attribute("@click").as_deref(), Some("open = ! open"));
	assert_eq!(
		el.to_string(),
		"<button @click=\"open = ! open\">Toggle</button>"
	);
}

#[test]
fn attr_many_alpine_like() {
	let html = "<div x-show=\"open\" x-transition:enter=\"transition ease-out duration-300\" x-transition:enter-start=\"opacity-0 scale-90\" x-transition:enter-end=\"opacity-100 scale-100\" x-transition:leave=\"transition ease-in duration-300\" x-transition:leave-start=\"opacity-100 scale-100\" x-transition:leave-end=\"opacity-0 scale-90\">Hello 👋</div>";
	let el = first_el(html);
	let mut e = (*el).clone();
	assert_eq!(e.get_attribute("x-show").as_deref(), Some("open"));
	assert_eq!(
		e.get_attribute("x-transition:enter").as_deref(),
		Some("transition ease-out duration-300")
	);
	assert_eq!(
		e.get_attribute("x-transition:enter-start").as_deref(),
		Some("opacity-0 scale-90")
	);
	assert_eq!(
		e.get_attribute("x-transition:enter-end").as_deref(),
		Some("opacity-100 scale-100")
	);
	assert_eq!(
		e.get_attribute("x-transition:leave").as_deref(),
		Some("transition ease-in duration-300")
	);
	assert_eq!(
		e.get_attribute("x-transition:leave-start").as_deref(),
		Some("opacity-100 scale-100")
	);
	assert_eq!(
		e.get_attribute("x-transition:leave-end").as_deref(),
		Some("opacity-0 scale-90")
	);
	assert_eq!(el.to_string(), html);
}
