use crate::define_css;

define_css!(heading, style!(r"
    display: flex;
    flex-direction: column;
    width: 100%;
    padding: 1.5em 2em;
    background-color: #eddfe8;
"));

define_css!(page_title, style!(r"
    color: #2e1424;
    font-weight: bold;
    font-size: 1.5em;
"));

define_css!(page_description, style!(r"
    color: #777074;
    font-size: 1em;
"));
