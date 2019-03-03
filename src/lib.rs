use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn md_to_html(text: String) -> PyResult<String> {
    let mut v = vec![];

    text.split("\n").for_each(|r| {
        let mut s = r.trim().to_string();

        if s.starts_with("**") {
            s = s.replacen("**", "<strong>", 1);
        }
        else if s.starts_with("*") {
            s = s.replacen("*", "<em>", 1);
        }
        else if s.starts_with("__") {
            s = s.replacen("__", "<strong>", 1);
        }
        else if s.starts_with("_") {
            s = s.replacen("_", "<em>", 1);
        }
        else if s.starts_with("`") {
            s = s.replacen("`", "<code>", 1);
        }

        s = s.replace(" **", " <strong>");
        s = s.replace("** ", "</strong> ");

        s = s.replace(" __", " <strong>");
        s = s.replace("__ ", "</strong> ");

        s = s.replace(" *", " <em>");
        s = s.replace("* ", "</em> ");

        s = s.replace(" _", " <em>");
        s = s.replace("_ ", "</em> ");

        s = s.replace(" `", " <code>");
        s = s.replace("` ", "</code> ");

        let s2 = s.trim_start_matches("#");

        let diff = s.len() - s2.len();

        if 6 >= diff && diff > 0 && &s2[0..1] == " " {
            s = format!("<h{0}>{1}</h{0}>", diff, s2.trim());
        }

        v.push(s);
    });

    return Ok(v.join("<br>\n"))
}


#[pymodule]
fn md_html(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(md_to_html))?;

    Ok(())
}