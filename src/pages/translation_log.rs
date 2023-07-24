static TRANSLATION_LOG_TEMPLATE: &'static str = r#"
        <div class=\"tl-outer\">
            <div class=\"tl-label-outer\">
                <div class=\"tl-label-inner\">{}</div>
                <div class=\"tl-label-inner\">{}</div>
                <div class=\"tl-label-inner lang-flow\">
                    <span class=\"tl-lang-original\">{}</span>{}
                </div>
            </div>
            <div class=\"tl-translation-outer\">
                <span class=\"tl-type-label original\">Original:</span>
                <span class=\"tl-content\">{}</span>
            </div>
            <div class=\"tl-translation-outer\">
                <span class=\"tl-type-label translated\">Translated:</span>
                <span class=\"tl-content\">{}</span>
            </div>
        </div>
"#;
