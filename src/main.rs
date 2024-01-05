use std::{fs::File, io::Write};

use cursive::{
    view::{Nameable, Resizable, Scrollable, SizeConstraint},
    views::{
        Dialog, DummyView, EditView, LinearLayout, ResizedView, ScrollView, SelectView, TextView,
    },
    Cursive, View,
};
use eyre::Result;
use parser::parse_languagelist;

mod parser;
const LANGUAGE_LIST: &[u8] = include_bytes!("../res/languagelist");

fn main() -> Result<()> {
    let mut siv = cursive::default();
    let lang = get_language_list()?;

    siv.add_layer(seatch_select_view_by_locales(lang));
    siv.run();

    Ok(())
}

fn get_language_list() -> Result<Vec<(String, String, String)>> {
    let mut lang = parse_languagelist(LANGUAGE_LIST)?
        .1
        .iter()
        .map(|x| (x.0.to_string(), x.1.to_string(), x.2.to_string()))
        .collect::<Vec<_>>();

    lang.sort_unstable();
    for i in ["English", "Chinese (Traditional)", "Chinese (Simplified)"] {
        let pos = lang.iter().position(|x| x.2 == i).unwrap();
        let entry = lang.remove(pos);
        lang.insert(0, entry);
    }

    Ok(lang)
}

fn search_fn_locales(items: Vec<(String, String, String)>, query: &str) -> Vec<String> {
    items
        .into_iter()
        .filter(|item| {
            let (lang, locale, lang_english) = item;
            let lang = lang.to_lowercase();
            let locale = locale.to_lowercase();
            let lang_english = lang_english.to_lowercase();
            let query = query.to_lowercase();

            lang.contains(&query) || locale.contains(&query) || lang_english.contains(&query)
        })
        .map(|x| x.0.to_string())
        .collect()
}

fn seatch_select_view_by_locales(list: Vec<(String, String, String)>) -> Dialog {
    let list_clone = list.clone();
    let on_edit = move |siv: &mut Cursive, query: &str, _cursor: usize| {
        let matches = search_fn_locales(list_clone.clone(), query);
        // Update the `matches` view with the filtered array of cities
        siv.call_on_name("matches", |v: &mut SelectView| {
            v.clear();
            v.add_all_str(matches);
        });
    };

    wrap_in_dialog(
        LinearLayout::vertical()
            .child(TextView::new("Search locale"))
            .child(
                EditView::new()
                    // update results every time the query changes
                    .on_edit(on_edit)
                    .with_name("query"),
            )
            .child(DummyView {})
            .child(
                SelectView::new()
                    .with_all_str(list.iter().map(|x| x.0.to_string()).collect::<Vec<_>>())
                    .on_submit(move |s: &mut Cursive, item| {
                        let item = list
                            .iter()
                            .find(|x| x.0 == item)
                            .map(|x| x.1.to_string())
                            .unwrap();

                        match set_locale(&item) {
                            Ok(()) => return,
                            Err(e) => {
                                show_msg(s, &e.to_string());
                            }
                        }
                    })
                    .with_name("matches")
                    .scrollable(),
            )
            .fixed_height(10),
        format!("Select Your locale"),
        None,
    )
}

fn wrap_in_dialog<V: View, S: Into<String>>(inner: V, title: S, width: Option<usize>) -> Dialog {
    Dialog::around(ResizedView::new(
        SizeConstraint::AtMost(width.unwrap_or(64)),
        SizeConstraint::Free,
        ScrollView::new(inner),
    ))
    .padding_lrtb(2, 2, 1, 1)
    .title(title)
}

fn show_msg(siv: &mut Cursive, msg: &str) {
    siv.add_layer(
        Dialog::around(TextView::new(msg).max_width(80))
            .title("Select language")
            .button("OK", |s| {
                s.pop_layer();
            })
            .padding_lrtb(2, 2, 1, 1),
    );
}

fn set_locale(locale: &str) -> Result<()> {
    let mut f = File::create("/etc/locale.conf")?;
    f.write_all(b"LANG=")?;

    Ok(f.write_all(format!("{locale}\n").as_bytes())?)
}
