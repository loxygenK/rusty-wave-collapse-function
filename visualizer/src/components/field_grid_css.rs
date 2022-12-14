use crate::{css_fn, define_css};

css_fn!(field_grid(width: usize, height: usize) {
    format!(r"
        display: grid;
        width: min(80vw, 80vh);
        height: min(80vw, 80vh);
        grid-template-columns: repeat({}, 1fr);
        grid-template-rows: repeat({}, 1fr);
        margin: 2em;
        border: 5px dotted #aaa;

        & > *:nth-child(2n) {{
            background-color: #eee;
        }}
    ", width, height)
});
