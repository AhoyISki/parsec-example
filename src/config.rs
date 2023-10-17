use parsec_core::{prelude::*, widgets::StatusLineCfg};
use parsec_kak::Editor;
use parsec_term::{Ui, VertRule};

pub fn config() -> SessionCfg<Ui, Editor> {
    palette::set_main_cursor(CursorStyle::new(None, Form::new().on_cyan()));

    let print_cfg = PrintCfg::new()
        .with_scrolloff(5)
        .width_wrapped()
        .with_new_line_after_space_as('↩');

    let cfg = Session::config(parsec_term::Ui::default())
        .with_input(Editor::new())
        .with_print_cfg(print_cfg)
        .with_file_fn(|builder, _| {
            builder.push::<VertRule>();
            builder.push::<LineNumbers>();
        })
        .with_window_fn(|builder| {
            let status: StatusLineCfg = status!(
                [FileName] { File::name } " "
                [Mode] { Editor::mode } " "
                [Selections] { DynInput(selections_fmt) } " "
                [Coords] { DynInput(main_col) }
                [Separator] ":" [Coords] { DynInput(main_line) }
                [Separator] "/" [Coords] { File::len_lines }
            )
            .global();
            let (area, _) = builder.push_cfg(status);

            builder.push_cfg_to(CommandLine::cfg().left_with_percent(50), area);
        });

    palette::set_form("Mode", Form::new().green());

    cfg
}
