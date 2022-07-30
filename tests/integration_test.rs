use jabba_ctc;

#[test]
fn set_text_and_get_text_test() {
    let backup = jabba_ctc::clipboard::get_text();
    //
    let text = "";
    jabba_ctc::clipboard::set_text(text);
    assert_eq!(jabba_ctc::clipboard::get_text(), text);

    let text = "a";
    jabba_ctc::clipboard::set_text(text);
    assert_eq!(jabba_ctc::clipboard::get_text(), text);

    let text = "hello";
    jabba_ctc::clipboard::set_text(text);
    assert_eq!(jabba_ctc::clipboard::get_text(), text);
    //
    jabba_ctc::clipboard::set_text(&backup);
    assert_eq!(jabba_ctc::clipboard::get_text(), backup);
}
