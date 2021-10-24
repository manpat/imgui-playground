pub mod prelude;

use prelude::*;

fn main() -> Result<()> {
	std::env::set_var("RUST_BACKTRACE", "1");

	let mut engine = toybox::Engine::new("imgui")?;
	let mut model = Model {
		quit: false,
		text: String::new(),
		color: [0.5; 3],
	};

	let test_context = TestContext::new(&mut engine.input);
	engine.input.enter_context(test_context.context_id());

	let another_context = AnotherContext::new(&mut engine.input);
	engine.input.enter_context(another_context.context_id());

	'main: loop {
		engine.process_events();
		if engine.should_quit() || model.quit {
			break 'main
		}

		let ui = engine.imgui.frame();

		ui.main_menu_bar(|| {
			ui.menu("File", || {
				if imgui::MenuItem::new("Close")
					.shortcut("Ctrl+Q")
					.build(ui)
				{
					model.quit = true;
				}
			});
			ui.menu("Blah", || {
				ui.text("blah");

				ui.menu("Submenu", || {
					imgui::MenuItem::new("abc")
						.build(ui);
				})
			});
		});

		imgui::Window::new("Fuck")
			.always_auto_resize(true)
			.build(ui, || {
				ui.text("Some text");
				ui.label_text("label", &model.text);
				ui.input_text("blah", &mut model.text).build();

				imgui::ColorButton::new("colour", [1.0, 0.0, 1.0, 1.0]).build(ui);
				imgui::ColorButton::new("grey", [0.5, 0.5, 0.5, 1.0]).build(ui);
				imgui::ColorButton::new("dark grey", [0.2, 0.2, 0.2, 1.0]).build(ui);

				imgui::ColorPicker::new("picker", &mut model.color)
					.format(imgui::ColorFormat::Float)
					.build(ui);
			});


		imgui::Window::new("Input")
			.build(ui, || {
				for context in engine.input.contexts() {
					if let Some(_node) = imgui::TreeNode::new(context.name())
						.default_open(true)
						.push(ui)
					{
						ui.label_text("ID", format!("{:?}", context.id()));

						if let Some(_node) = imgui::TreeNode::new("Actions")
							.default_open(true)
							.push(ui)
						{
							for action in context.actions() {
								ui.text(action.name());
							}
						}
					}
				}
			});

		let input = engine.input.frame_state();

		if input.active(test_context.quit) {
			model.quit = true;
		}

		if input.active(test_context.some_trigger) {
			model.text.push_str("TRIGGER ");
		}

		let clear_color = Color::from(model.color).to_linear();
		engine.gfx.render_state().set_clear_color(clear_color);
		engine.gfx.render_state().clear(gfx::ClearMode::COLOR | gfx::ClearMode::DEPTH);

		engine.end_frame();
	}

	Ok(())
}


struct Model {
	quit: bool,
	text: String,
	color: [f32; 3],
}


toybox::declare_input_context! {
	struct TestContext "Test Context" {
		trigger some_trigger { "Some Trigger" [Scancode::Return] }
		trigger quit { "Quit" [Scancode::Escape] }
	}
}

toybox::declare_input_context! {
	struct AnotherContext "Another Context" {
		pointer mouse { "Mouse" }
	}
}